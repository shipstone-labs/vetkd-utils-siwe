import { writable } from "svelte/store";
import type { Subscription } from "@xstate/store";
import {
  SiweManager,
  siweStateStore,
  type SiweIdentityContextType,
} from "ic-siwe-js";
import type { ActorConfig, HttpAgentOptions } from "@dfinity/agent";

// ✅ Define the store with default values
function createSiweIdentityStore() {
  const store = writable<SiweIdentityContextType>({
    isInitializing: false,
    isPreparingLogin: false,
    isPrepareLoginError: false,
    isPrepareLoginSuccess: false,
    isPrepareLoginIdle: true,
    isLoggingIn: false,
    isLoginError: false,
    isLoginSuccess: false,
    isLoginIdle: true,
    prepareLogin: async () => {},
    login: async () => {
      return undefined;
    },
    clear: () => {},
    prepareLoginStatus: "idle",
    prepareLoginError: undefined,
    loginStatus: "idle",
    loginError: undefined,
    signMessageStatus: "idle",
    signMessageError: undefined,
    delegationChain: undefined,
    identity: undefined,
    identityAddress: undefined,
  });

  let siweManager: SiweManager;
  let unsubscribeSiweStore: Subscription | undefined = undefined;

  // ✅ Initialize the SIWE manager and sync the store with siweStateStore
  function init({
    canisterId,
    httpAgentOptions,
    actorOptions,
  }: {
    canisterId: string;
    httpAgentOptions?: HttpAgentOptions;
    actorOptions?: ActorConfig;
  }) {
    siweManager = new SiweManager(canisterId, httpAgentOptions, actorOptions);

    store.update((s) => ({
      ...s,
      prepareLogin: async () => await siweManager.prepareLogin(),
      login: async () => await siweManager.login(),
      clear: () => siweManager.clear(),
    }));

    // ✅ Subscribe to `siweStateStore` to sync state
    unsubscribeSiweStore = siweStateStore.subscribe(({ context }) => {
      store.update((s) => ({
        ...s,
        isInitializing: context.isInitializing,
        prepareLoginStatus: context.prepareLoginStatus,
        isPreparingLogin: context.prepareLoginStatus === "preparing",
        isPrepareLoginError: context.prepareLoginStatus === "error",
        isPrepareLoginSuccess: context.prepareLoginStatus === "success",
        isPrepareLoginIdle: context.prepareLoginStatus === "idle",
        prepareLoginError: context.prepareLoginError,
        loginStatus: context.loginStatus,
        isLoggingIn: context.loginStatus === "logging-in",
        isLoginError: context.loginStatus === "error",
        isLoginSuccess: context.loginStatus === "success",
        isLoginIdle: context.loginStatus === "idle",
        loginError: context.loginError,
        signMessageStatus: context.signMessageStatus,
        signMessageError: context.signMessageError,
        delegationChain: context.delegationChain,
        identity: context.identity,
        identityAddress: context.identityAddress,
      }));
    });
  }

  // ✅ Cleanup function to unsubscribe
  function destroy() {
    if (unsubscribeSiweStore) {
      unsubscribeSiweStore.unsubscribe();
      unsubscribeSiweStore = undefined;
    }
  }

  return {
    subscribe: store.subscribe,
    init,
    destroy,
  };
}

// ✅ Export the store
export const siweIdentityStore = createSiweIdentityStore();
