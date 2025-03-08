<script lang="ts">
import { onDestroy } from "svelte";
import { Editor, placeholder } from "typewriter-editor";
import { draft } from "../store/draft";
import { addNote, refreshNotes } from "../store/notes";
import { addNotification, showError } from "../store/notifications";
import Header from "./Header.svelte";
import NoteEditor from "./NoteEditor.svelte";
import TagEditor from "./TagEditor.svelte";
import { noteFromContent } from "@shipstone-labs/ic-vetkd-notes-client";
import { siweIdentityStore } from "../store/siwe";

let creating = false;
let tags: string[] = $draft.tags;

const editor = new Editor({
	modules: {
		placeholder: placeholder("Start typing..."),
	},
	html: $draft.content,
});

const store = siweIdentityStore.store;

async function add() {
	const { isLoginSuccess, identity, actor, crypto } = $store;
	if (!isLoginSuccess || !identity || !actor || !crypto) {
		return;
	}
	creating = true;
	await addNote(
		noteFromContent(editor.getHTML(), tags, identity.getPrincipal()),
		actor,
		crypto,
	)
		.catch((e) => {
			showError(e, "Could not add note.");
		})
		.finally(() => {
			creating = false;
		});

	// if creation was successful, reset the editor
	editor.setHTML("");
	tags = [];

	addNotification({ type: "success", message: "IP Doc added successfully" });

	// refresh notes in the background
	refreshNotes(actor, crypto).catch((e) =>
		showError(e, "Could not refresh notes."),
	);
}

function saveDraft() {
	draft.set({
		content: editor.getHTML(),
		tags: tags,
	});
}

function addTag(tag: string) {
	tags = [...tags, tag];
}

function removeTag(tag: string) {
	tags = tags.filter((t) => t !== tag);
}

onDestroy(saveDraft);
</script>

<svelte:window on:beforeunload={saveDraft} />

<Header>
  <span slot="title"> New IP Doc </span>
</Header>

<main class="p-4">
  <NoteEditor {editor} class="mb-3" disabled={creating} />
  <div class="bg-gray-100 dark:bg-base-100 p-4 rounded-lg shadow-md space-y-2 text-sm">
    <div class="flex flex-row">
      <span class="font-bold w-28">Tags:</span>
      <span>
        <TagEditor
          {tags}
          on:add={(e) => addTag(e.detail)}
          on:remove={(e) => removeTag(e.detail)}
          disabled={creating}
        />
      </span>
    </div>
  </div>
  <button
    class="btn mt-6 btn-primary {creating ? 'loading' : ''}"
    disabled={creating}
    on:click={add}>{creating ? 'Adding...' : 'Add note'}</button
  >
</main>

<style>
</style>
