import { get, writable } from "svelte/store";
import {
  type BackendActor,
  createActor,
  CryptoService,
  sleep,
} from "@shipstone-labs/ic-vetkd-notes-client";
import { AuthClient } from "@dfinity/auth-client";
import { addNotification, showError } from "./notifications";
import type { JsonnableDelegationChain } from "@dfinity/identity/lib/cjs/identity/delegation";
import { navigateTo } from "svelte-router-spa";
import { doLogin } from "../lib/web3modal";

// if (typeof global === "undefined") {
//   var global = window; // ✅ Ensures `global` exists in browser
// }

export type AuthState =
  | {
      state: "initializing-auth";
      crypto?: undefined;
      actor?: undefined;
      client?: undefined;
    }
  | {
      state: "anonymous";
      actor: BackendActor;
      crypto?: undefined;
      client: AuthClient;
    }
  | {
      state: "initializing-crypto";
      actor: BackendActor;
      crypto?: undefined;
      client: AuthClient;
    }
  | {
      state: "synchronizing";
      actor: BackendActor;
      crypto?: undefined;
      client: AuthClient;
    }
  | {
      state: "initialized";
      actor: BackendActor;
      client: AuthClient;
      crypto: CryptoService;
    }
  | {
      state: "error";
      actor?: undefined;
      crypto?: undefined;
      client?: undefined;
      error: string;
    };

export const auth = writable<AuthState>({
  state: "initializing-auth",
});

async function initAuth() {
  const client = await AuthClient.create();
  if (await client.isAuthenticated()) {
    authenticate(client);
  } else {
    auth.update(() => ({
      state: "anonymous",
      actor: createActor(),
      client,
    }));
  }
}

initAuth();

export function login() {
  const currentAuth = get(auth);

  if (currentAuth.state === "anonymous") {
    doLogin("Please sign this message to authenticate with the app.");
    // currentAuth.client.login({
    //   maxTimeToLive: BigInt(1800) * BigInt(1_000_000_000),
    //   identityProvider:
    //     process.env.DFX_NETWORK === "ic"
    //       ? "https://identity.ic0.app/#authorize"
    //       : `http://${process.env.INTERNET_IDENTITY_CANISTER_ID}.localhost:8000/#authorize`,
    //   // `http://localhost:8000?canisterId=${process.env.INTERNET_IDENTITY_CANISTER_ID}#authorize`,
    //   onSuccess: () => authenticate(currentAuth.client),
    // });
  }
}

export async function logout() {
  const currentAuth = get(auth);

  if (currentAuth.state === "initialized") {
    await currentAuth.client.logout();
    auth.update(() => ({
      state: "anonymous",
      actor: createActor(),
      client: currentAuth.client,
    }));
    navigateTo("/");
  }
}

export async function authenticate(client: AuthClient) {
  handleSessionTimeout();

  try {
    const actor = createActor({
      agentOptions: {
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        identity: client.getIdentity() as any,
      },
    });

    auth.update(() => ({
      state: "initializing-crypto",
      actor,
      client,
    }));

    const cryptoService = new CryptoService(actor);

    auth.update(() => ({
      state: "initialized",
      actor,
      client,
      crypto: cryptoService,
    }));
  } catch (e) {
    auth.update(() => ({
      state: "error",
      error: (e as Error).message || "An error occurred",
    }));
  }
}

// set a timer when the II session will expire and log the user out
function handleSessionTimeout() {
  // upon login the localstorage items may not be set, wait for next tick
  setTimeout(() => {
    try {
      const delegation = JSON.parse(
        window.localStorage.getItem("ic-delegation") || "null"
      ) as JsonnableDelegationChain;

      if (!delegation) {
        return;
      }
      const expirationTimeMs =
        Number.parseInt(delegation.delegations[0].delegation.expiration, 16) /
        1000000;

      setTimeout(() => {
        logout();
      }, expirationTimeMs - Date.now());
    } catch (error) {
      console.error("Could not handle delegation expiry.", error);
    }
  });
}
