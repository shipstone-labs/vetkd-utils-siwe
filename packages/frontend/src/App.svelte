<script lang="ts">
import Hero from "./components/Hero.svelte";
import LayoutAuthenticated from "./components/LayoutAuthenticated.svelte";
import Notifications from "./components/Notifications.svelte";
import { auth } from "./store/auth";

import { account } from "./lib/web3modal";
import Network from "../partials/Network.svelte";
import SignMessage from "../partials/SignMessage.svelte";
import SignTypeData from "../partials/SignTypeData.svelte";
import Transaction from "../partials/Transaction.svelte";
import Wallet from "../partials/Wallet.svelte";
import CustomForm from "../partials/CustomForm.svelte";

import { onDestroy } from "svelte";
import { siweIdentityStore } from "./store/siwe";

// ✅ Initialize SIWE with your canisterId
siweIdentityStore.init({
	canisterId: process.env.IC_SIWE_PROVIDER_CANISTER_ID || "",
});

// ✅ Cleanup when component is destroyed
onDestroy(() => {
	siweIdentityStore.destroy();
});
</script>

{#if $auth.state === 'initialized'}
  <LayoutAuthenticated />
{:else}
  <Hero auth={$auth} />
{/if}
<Notifications />

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
