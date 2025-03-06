<script lang="ts">
import type { NoteModel } from "@shipstone-labs/vetkd-notes-client";
import { addUser, refreshNotes, removeUser } from "../store/notes";
import { addNotification, showError } from "../store/notifications";
import { siweIdentityStore } from "../store/siwe";

export let editedNote: NoteModel;
// biome-ignore lint/style/useConst: <explanation>
export let ownedByMe = false;

import { createEventDispatcher } from "svelte";

const store = siweIdentityStore.store;
const dispatch = createEventDispatcher();

let newSharing = "";
// biome-ignore lint/style/useConst: <explanation>
let newWhenValue = "";
// biome-ignore lint/style/useConst: <explanation>
let newWhenChecked = true;
// biome-ignore lint/style/useConst: <explanation>
let newSharingChecked = false;
let newSharingInput: HTMLInputElement;
let newWhenInput: HTMLInputElement;
let adding = false;
let removing = false;

function dateValue(input: string): bigint | null {
	if (!input) {
		return null;
	}
	const date = new Date(input);
	return BigInt(date.valueOf()) * BigInt(1000000);
}

async function add() {
	if (!$store.actor || !$store.crypto) {
		throw new Error("Not authenticated");
	}
	adding = true;
	try {
		await addUser(
			editedNote.id,
			newSharingChecked ? null : newSharing || null,
			dateValue(newWhenValue),
			$store.actor,
		);
		addNotification({
			type: "success",
			message: "User successfully added",
		});
		const value = dateValue(newWhenValue);
		editedNote.users = [
			...editedNote.users.filter(
				(u: string[]) => u[0] !== (newSharing || "everyone"),
			),
			[newSharing, { when: value ? [value] : [], was_read: false }],
		];
		const when = newWhenChecked
			? null
			: value
				? Number(value / BigInt(1000000))
				: null;
		dispatch("message", {
			action: "share",
			user: newSharingChecked ? null : newSharing || "everyone",
			rule: newSharingChecked
				? [
						"everyone",
						{ when: newWhenChecked && value ? [value] : [], was_read: false },
					]
				: [
						newSharing,
						{ when: newWhenChecked && value ? [value] : [], was_read: false },
					],
			created_at: BigInt(Date.now()) * BigInt(1000000),
		});
		newSharing = "";
		newSharingChecked = false;
		newSharingInput.focus();
		newWhenChecked = true;
	} catch (e) {
		showError(e, "Could not add user.");
	} finally {
		adding = false;
	}
	await refreshNotes($store.actor, $store.crypto).catch((e) =>
		showError(e, "Could not refresh notes."),
	);
}

async function remove(sharing: string) {
	if (!$store.actor || !$store.crypto) {
		throw new Error("Not authenticated");
	}
	removing = true;
	try {
		await removeUser(editedNote.id, sharing, $store.actor);
		editedNote.users = editedNote.users.filter(
			(u: string[]) => (u[0] || "everyone") !== (sharing || "everyone"),
		);
		addNotification({
			type: "success",
			message: "User successfully removed",
		});
		dispatch("message", {
			action: "unshare",
			user: sharing || "everyone",
			when: null,
			createdAt: Date.now(),
		});
	} catch (e) {
		showError(e, "Could not remove user.");
	} finally {
		removing = false;
	}
	await refreshNotes($store.actor, $store.crypto).catch((e) =>
		showError(e, "Could not refresh notes."),
	);
}
</script>

<div class="bg-gray-100 dark:bg-base-100 p-4 rounded-lg shadow-md">
  <p class="text-lg font-bold mb-2">Sharing IP Docs</p>
  {#if ownedByMe}
    <p class="mt-1">
      Add users by their principal or everyone to allow them to read the IP Doc.
      Optionally you can set a date at which the note will become readable by them.
      Each user has their principal with a copy button at the top left of the page.
    </p>
  {:else}
    <p class="mt-3">
      This note is <span class="font-bold">shared</span> with you. It is owned
      by <span class="italic font-bold">{editedNote.owner}</span>.
    </p>
    <p class="mt-3">Users with whom the owner shared the note:</p>
  {/if}
  <div class="flex flex-col gap-2 mt-2">
    {#each editedNote.users as sharing}
      <div class="flex flex-row">
        <button
          class="btn btn-outline btn-sm flex flex-row items-center gap-2 space-2"
          on:click={() => {
            if (!sharing[1]?.was_read) {
              remove(sharing[0]);
            }
          }}
          disabled={adding || removing || !ownedByMe || sharing[1]?.was_read}
        >
          <span><b>Who:</b> {sharing[0] || "everyone"}</span>
          <span><b>When:</b> {sharing[1]?.when[0] ? (new Date(Number(sharing[1].when[0] / BigInt(1000000)))).toLocaleString() : "always"}</span>
          {#if !sharing[1]?.was_read}<svg width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <line x1="3" y1="3" x2="21" y2="21" stroke="currentColor" stroke-width="2"/>
              <line x1="3" y1="21" x2="21" y2="3" stroke="currentColor" stroke-width="2"/>
          </svg>{/if}
        </button>
      </div>
    {/each}
    <div class="flex flex-row text-sm">
      <label class="inline-flex items-center mx-3 font-normal {!ownedByMe ? 'hidden' : ''}">
        <span class="mx-1 font-bold">Who:</span>
        <input
          type="checkbox"
          bind:checked={newSharingChecked}
          class="mx-1"
          disabled={adding || removing}/> everyone</label>
      <input
        bind:value={newSharing}
        placeholder="Add principal..."
        class="mx-3 bg-transparent text-base rounded-lg h-8 px-3 w-auto {adding ||
        removing
          ? 'opacity-50'
          : ''} 
          {!ownedByMe || newSharingChecked ? 'hidden' : ''}"
        bind:this={newSharingInput}
        disabled={adding}
      />
      <label class="inline-flex items-center mx-3 font-normal {!ownedByMe ? 'hidden' : ''}">
        <span class="mx-1 font-bold">When:</span>
        <input
          type="checkbox"
          bind:checked={newWhenChecked}
          class="mx-1"
          disabled={adding || removing}/>always</label>
      <input
        bind:value={newWhenValue}
        placeholder="Add date..."
        type="datetime-local"
        class="mx-3 bg-transparent text-base rounded-lg h-8 px-3 w-auto {adding ||
        removing
          ? 'opacity-50'
          : ''} 
          {!ownedByMe || newWhenChecked ? 'hidden' : ''}"
        bind:this={newWhenInput}
        disabled={adding || newWhenChecked}
      />
      <button
        class="mx-3 btn btn-sm btn-ghost
          {!ownedByMe ? 'hidden' : ''}
          {adding || removing ? 'loading' : ''}"
        on:click={add}
        disabled={editedNote.users.find((u: string[]) => (u[0] || "everyone") === (newSharing || "everyone")) != null ||
          adding ||
          removing}
        >{adding ? 'Adding...' : removing ? 'Removing... ' : 'Add'}</button
      >
    </div>
  </div>
</div>
