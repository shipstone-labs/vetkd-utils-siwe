import { writable } from "svelte/store";
import {
  type BackendActor,
  type EncryptedNote,
  type CryptoService,
  deserialize,
  type NoteModel,
  serialize,
} from "@shipstone-labs/ic-vetkd-notes-client";
import { showError } from "./notifications";
import { siweIdentityStore } from "./siwe";

const store = siweIdentityStore.store;

export const notesStore = writable<
  | {
      state: "uninitialized";
    }
  | {
      state: "loading";
    }
  | {
      state: "loaded";
      list: NoteModel[];
    }
  | {
      state: "error";
    }
>({ state: "uninitialized" });

let notePollerHandle: ReturnType<typeof setInterval> | null;

async function decryptNotes(
  notes: EncryptedNote[],
  cryptoService: CryptoService,
  actor: BackendActor
): Promise<NoteModel[]> {
  return await Promise.all(
    notes.map((encryptedNote) =>
      deserialize(encryptedNote, cryptoService, actor)
    )
  );
}

function updateNotes(notes: NoteModel[]) {
  notesStore.set({
    state: "loaded",
    list: notes,
  });
}

export async function refreshNotes(
  actor: BackendActor,
  cryptoService: CryptoService
) {
  const encryptedNotes = await actor.get_notes();

  const notes = await decryptNotes(encryptedNotes, cryptoService, actor);
  updateNotes(notes);
}

export async function addNote(
  note: NoteModel,
  actor: BackendActor,
  crypto: CryptoService
) {
  const new_id: bigint = await actor.create_note();
  note.id = new_id;
  const { encrypted_text: encryptedNote, data } = await serialize(note, crypto);
  await actor.update_note(new_id, data, encryptedNote);
}

export async function updateNote(
  note: NoteModel,
  actor: BackendActor,
  crypto: CryptoService
) {
  const { encrypted_text: encryptedNote, data } = await serialize(note, crypto);
  await actor.update_note(note.id, data, encryptedNote);
}

export async function addUser(
  id: bigint,
  user: string | null,
  when: bigint | null,
  actor: BackendActor
) {
  await actor.add_user(id, user ? [user] : [], when ? [when] : []);
}

export async function removeUser(
  id: bigint,
  user: string,
  actor: BackendActor
) {
  await actor.remove_user(id, user ? [user] : []);
}

store.subscribe(async ($store) => {
  const { identity, actor, crypto, isLoginSuccess } = $store;
  if (identity && actor && crypto && isLoginSuccess) {
    if (notePollerHandle !== null) {
      clearInterval(notePollerHandle);
      notePollerHandle = null;
    }

    notesStore.set({
      state: "loading",
    });
    try {
      await refreshNotes(actor, crypto).catch((e) =>
        showError(e, "Could not poll notes.")
      );

      notePollerHandle = setInterval(async () => {
        await refreshNotes(actor, crypto).catch((e) =>
          showError(e, "Could not poll notes.")
        );
      }, 3000);
    } catch {
      notesStore.set({
        state: "error",
      });
    }
  } else if ((!isLoginSuccess || !identity) && notePollerHandle !== null) {
    clearInterval(notePollerHandle);
    notePollerHandle = null;
    notesStore.set({
      state: "uninitialized",
    });
  }
});
