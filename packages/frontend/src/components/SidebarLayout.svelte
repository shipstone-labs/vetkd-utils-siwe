<script lang="ts">
import FaPlusSquare from "svelte-icons/fa/FaPlusSquare.svelte";
import FaBook from "svelte-icons/fa/FaBook.svelte";
import FaMobileAlt from "svelte-icons/fa/FaMobileAlt.svelte";
import FaDoorOpen from "svelte-icons/fa/FaDoorOpen.svelte";
import Disclaimer from "./Disclaimer.svelte";
import { copy } from "svelte-copy";
import { siweIdentityStore } from "../store/siwe";
const store = siweIdentityStore.store;
</script>

<div class="bg-white dark:bg-base-200 drawer drawer-mobile min-h-screen">
  <input id="my-drawer-3" type="checkbox" class="drawer-toggle" />
  <div class="flex flex-col drawer-content lg:!z-[1000]">
    <div class="flex-1">
      <slot />
    </div>
    <Disclaimer />
  </div>
  <div class="drawer-side">
    <label for="my-drawer-3" class="drawer-overlay" />
    <aside
      class="flex flex-col justify-between border-r border-base-300 bg-base-100 text-base-content w-64 sm:w-80 "
    >
      <div
        class="sticky h-16 py-4 pl-5 text-2xl font-bold border-b border-base-300 text-primary dark:text-white"
      >
        <div class="hidden dark:flex pl-4 flex-shrink-0">
          <img
            src="/rendered.svg"
            alt="Shipstone Labs"
            class="h-5 sm:h-7 md:h-10 w-auto pe-3 z-20"
          />
        </div>
        <div class="pl-4 dark:hidden flex-shrink-0">
          <img
            src="/rendered-light.svg"
            alt="Shipstone Labs"
            class="h-5 sm:h-7 md:h-10 w-auto pe-3 z-20"
          />
        </div>
      </div>
      <div class="border-b">
        <div class="pl-4">My Principal</div>
        <div class="pl-4"><small>{$store.identity?.getPrincipal()}</small><button use:copy="{$store.identity?.getPrincipal().toString()}"><svg width="18px" height="18px" viewBox="0 0 24 24" id="magicoon-Filled" xmlns="http://www.w3.org/2000/svg"><title>copy</title><g id="copy-Filled"><path id="copy-Filled-2" data-name="copy-Filled" fill="currentColor" d="M11,19.5h5.7A3.978,3.978,0,0,1,13,22H6a4,4,0,0,1-4-4V10A4.007,4.007,0,0,1,5.5,6.03V14A5.51,5.51,0,0,0,11,19.5ZM19.5,6.25h2.12A1.638,1.638,0,0,0,21.41,6L18,2.59a1.156,1.156,0,0,0-.25-.2V4.5A1.758,1.758,0,0,0,19.5,6.25Zm0,1.5A3.256,3.256,0,0,1,16.25,4.5V2H11A4,4,0,0,0,7,6v8a4,4,0,0,0,4,4h7a4,4,0,0,0,4-4V7.75Z"/></g></svg></button></div>
      </div>
      <ul
        class="p-4 overflow-y-auto menu w-full bg-base-100 flex-1 flex flex-col"
      >
        <li>
          <a href="/">
            <span class="w-6 h-6 p-1 mr-2">
              <FaPlusSquare />
            </span>
            New IP Doc
          </a>
        </li>
        <li>
          <a href="/notes">
            <span class="w-6 h-6 p-1 mr-2">
              <FaBook />
            </span>
            Your IP Docs</a
          >
        </li>
        <li class="flex-1" />
        <li>
          <button on:click={() => $store.clear()}>
            <span class="w-6 h-6 p-1 mr-2">
              <FaDoorOpen />
            </span>
            Log out</button
          >
        </li>
      </ul>
      <div class="px-5 pb-4">
        <img
          src="/img/ic-badge-powered-by-crypto_transparent-white-text.png"
          alt="Powered by the Internet Computer"
          class="hidden dark:inline "
        />
        <img
          src="/img/ic-badge-powered-by-crypto_transparent-dark-text.png"
          alt="Powered by the Internet Computer"
          class="dark:hidden inline "
        />
      </div>
    </aside>
  </div>
</div>
