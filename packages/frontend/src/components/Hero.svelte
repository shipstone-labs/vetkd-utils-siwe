<script lang="ts">
import { type AuthState, login } from "../store/auth";
import DisclaimerCopy from "./DisclaimerCopy.svelte";
import Spinner from "./Spinner.svelte";
import "../lib/web3modal";
import Header from "./Header.svelte";

export let auth: Extract<
	AuthState,
	{
		state:
			| "initializing-auth"
			| "initializing-crypto"
			| "synchronizing"
			| "anonymous"
			| "error";
	}
>;
</script>

<Header/>
<div class="hero min-h-screen pt-8 sm:pt-0 content-start sm:content-center">
  <div class="text-center hero-content ">
    <div class="max-w-xl ">
      <h1
        class="mb-5 text-4xl sm:text-5xl font-bold text-primary dark:text-white"
      >
        Mini IP Manager
      </h1>
      <p class="mb-5 text-xl font-semibold">
        Your private IP Docs on the Internet Computer.
      </p>
      <p class="mb-5">
        A safe place to store and share your personal IP.
      </p>

      {#if auth.state === 'initializing-auth' || auth.state === 'initializing-crypto'}
        <div class="text-lg font-semibold mt-8">
          <Spinner />
          Initializing...
        </div>
      {:else if auth.state === 'synchronizing'}
        <div class="text-lg font-semibold">
          <Spinner />
          Synchronizing... Please keep the app open on a device that's already added.
        </div>
      {:else if auth.state === 'anonymous'}
        <button class="btn btn-primary" on:click={() => login()}
          >Login to Store and Share your IP</button
        >
      {:else if auth.state === 'error'}
        <div class="text-lg font-semibold mt-8">An error occurred.</div>
      {/if}

      <div class="text-xs mt-8 sm:mt-12 opacity-75 mb-12 sm:mb-32">
        <DisclaimerCopy />
      </div>
    </div>
  </div>
  <div class="fixed bottom-0 text-center left-0 right-0 pb-4 sm:pb-8">
    <img
      src="/img/ic-badge-powered-by-crypto_label-stripe-white-text.png"
      alt="Powered by the Internet Computer"
      class="hidden dark:inline h-4"
    />
    <img
      src="/img/ic-badge-powered-by-crypto_label-stripe-dark-text.png"
      alt="Powered by the Internet Computer"
      class="dark:hidden inline h-4"
    />
  </div>
</div>
