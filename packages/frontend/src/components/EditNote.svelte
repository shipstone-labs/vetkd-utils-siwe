<script lang="ts">
import { navigateTo } from "svelte-router-spa";
import type { CurrentRoute } from "svelte-router-spa/types/components/route";
import { Editor, placeholder } from "typewriter-editor";
import {
	extractTitle,
	type HistoryEntry,
	type NoteModel,
} from "@shipstone-labs/ic-vetkd-notes-client";
import {
	notesStore,
	refreshNotes,
	updateNote,
	addUser,
	removeUser,
} from "../store/notes";
import Header from "./Header.svelte";
import NoteEditor from "./NoteEditor.svelte";
import TagEditor from "./TagEditor.svelte";
import SharingEditor from "./SharingEditor.svelte";
import Trash from "svelte-icons/fa/FaTrash.svelte";
import { addNotification, showError } from "../store/notifications";
import Spinner from "./Spinner.svelte";
import { siweIdentityStore } from "../store/siwe";

export let currentRoute: CurrentRoute;

let editedNote: NoteModel;
let editor: Editor;
let updating = false;
let deleting = false;
const store = siweIdentityStore.store;
// biome-ignore lint/style/useConst: <explanation>
export let history: HistoryEntry[] = [];
export let ownedByMe = true;

function addHistory(e: CustomEvent<HistoryEntry>) {
	history = [...history, { ...e.detail }];
}

async function save() {
	const { isLoginSuccess, identityAddress, actor, crypto } = $store;
	if (!isLoginSuccess || !identityAddress || !actor || !crypto) {
		return;
	}
	const html = editor.getHTML();
	updating = true;
	await updateNote(
		{
			...editedNote,
			content: html,
			title: extractTitle(html),
			updatedAt: Date.now(),
		},
		actor,
		crypto,
	)
		.catch((e) => {
			showError(e, "Could not update note.");
		})
		.finally(() => {
			updating = false;
			addHistory(
				new CustomEvent("history", {
					detail: {
						action: "updated",
						user: selfPrincipalString(),
						rule: [],
						labels: [],
						created_at: BigInt(Date.now() * 1000000),
					},
				}),
			);
		});

	addNotification({ type: "success", message: "IP Doc saved successfully" });

	await refreshNotes(actor, crypto).catch((e) =>
		showError(e, "Could not refresh IP Doc."),
	);
}

async function deleteNote() {
	const { isLoginSuccess, identityAddress, actor, crypto } = $store;
	if (!isLoginSuccess || !identityAddress || !actor || !crypto) {
		return;
	}
	deleting = true;
	await actor.delete_note(editedNote.id).catch((e: MouseEvent) => {
		deleting = false;
		showError(e, "Could not delete IP Doc.");
	});

	await refreshNotes(actor, crypto)
		.catch((e) => showError(e, "Could not refresh IP Docs."))
		.finally(() => {
			addNotification({
				type: "success",
				message: "IP Doc deleted successfully",
			});
			navigateTo("/notes");
		});
}

function addTag(tag: string) {
	editedNote.tags = [...editedNote.tags, tag];
}

function removeTag(tag: string) {
	editedNote.tags = editedNote.tags.filter((t) => t !== tag);
}

function selfPrincipalString(): string {
	const { isLoginSuccess, identity, actor, crypto } = $store;
	if (!isLoginSuccess || !identity || !actor || !crypto) {
		return "";
	}
	return identity.getPrincipal().toString();
}

$: {
	if ($notesStore.state === "loaded" && !editedNote) {
		const note = $notesStore.list.find(
			(note) => note.id.toString() === currentRoute.namedParams.id,
		);

		if (note) {
			editedNote = { ...note };
			history = note.history;
			editor = new Editor({
				modules: {
					placeholder: placeholder("Start typing..."),
				},
				html: editedNote.content,
			});
			ownedByMe = note.owner === selfPrincipalString();
		}
	}
}
</script>

{#if editedNote}
  <Header>
    <span slot="title"> Edit note </span>
    <button
      slot="actions"
      class="btn btn-ghost {deleting ? 'loading' : ''} {!ownedByMe || editedNote.locked ? 'hidden' : ''}"
      on:click={deleteNote}
      disabled={updating || deleting}
    >
      {#if !deleting}
        <span class="w-6 h-6 p-1"><Trash /></span>
      {/if}

      {deleting ? 'Deleting...' : ''}
    </button>
  </Header>
  <main class="p-4 space-y-6">
    {#if $notesStore.state === 'loaded'}
      <!-- Note Editor Section -->
      <NoteEditor {editor} disabled={updating || deleting || !ownedByMe || editedNote.locked} class="mb-4" />
      
      <!-- Note Details Section -->
      <div class="bg-gray-100 dark:bg-base-100 p-4 rounded-lg shadow-md space-y-2 text-sm">
        <div class="flex flex-row">
          <span class="font-bold w-28">Created:</span>
          <span>{new Date(editedNote.createdAt).toLocaleString()}</span>
        </div>
        <div class="flex flex-row">
          <span class="font-bold w-28">Updated:</span>
          <span>{new Date(editedNote.updatedAt).toLocaleString()}</span>
        </div>
        <div class="flex flex-row">
          <span class="font-bold w-28">Status:</span>
          <span>{!ownedByMe || editedNote.locked || editedNote.locked ? 'ReadOnly' : 'Editable' }</span>
        </div>
        <div class="flex flex-row">
          <span class="font-bold w-28">Tags:</span>
          <span>
            <TagEditor
              tags={editedNote.tags}
              on:add={(e) => addTag(e.detail)}
              on:remove={(e) => removeTag(e.detail)}
              disabled={updating || deleting || !ownedByMe || editedNote.locked}
            />
          </span>
        </div>
      </div>

      <!-- Action Buttons and Sharing Section -->
      <div class="space-y-4">
        <button
          class="btn btn-primary {updating ? 'loading' : ''} w-full md:w-auto"
          disabled={updating || deleting || !ownedByMe || editedNote.locked}
          on:click={save}>
          {updating ? 'Saving...' : 'Save'}
        </button>

        <SharingEditor {editedNote} {ownedByMe} on:message={addHistory}/>
      </div>

      <!-- History Section -->
      <div class="bg-gray-100 dark:bg-base-100 p-4 rounded-lg shadow-md">
        <p class="text-lg font-bold mb-2">History</p>
        <div class="space-y-1">
          {#each history as entry}
            <div class="flex flex-row bg-gray-200 dark:bg-base-200 space-x-4 text-sm p-2 rounded-lg shadow-md">
              <span class="font-mono text-gray-600 dark:text-white text-xs">{new Date(Number(entry.created_at / BigInt(1000000))).toLocaleDateString()}<br/>{new Date(Number(entry.created_at / BigInt(1000000))).toLocaleTimeString()}</span>
              <span>
                {entry.action}
                {entry.rule ? entry.rule.map(([user, when]) => `${user} (${when?.[0] ? `after ${new Date(Number(when[0]/ BigInt(1000000))).toLocaleString()}` : "always"})`).join(', ') : ""}
              </span>
            </div>
          {/each}
        </div>
      </div>
    
    {:else if $notesStore.state === 'loading'}
      <!-- Loading State -->
      <div class="text-center text-lg font-semibold">Loading IP Docs...</div>
    {/if}
  </main>
{:else}
  <Header>
    <span slot="title"> Edit IP Doc </span>
  </Header>
  <main class="p-4">
    {#if $notesStore.state === 'loading'}
      <Spinner />
      Loading IP Doc...
    {:else if $notesStore.state === 'loaded'}
      <div class="alert alert-error">Could not find IP Doc.</div>
    {/if}
  </main>
{/if}
