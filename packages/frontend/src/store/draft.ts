import { writable } from "svelte/store";
import { siweIdentityStore } from "./siwe";

const store = siweIdentityStore.store;

interface DraftModel {
  content: string;
  tags: string[];
}

let initialDraft: DraftModel = {
  content: "",
  tags: [],
};

try {
  const savedDraft: DraftModel = JSON.parse(
    localStorage.getItem("draft") || "{}"
  );
  if ("content" in savedDraft && "tags" in savedDraft) {
    initialDraft = savedDraft;
  }
} catch {}

export const draft = writable<DraftModel>(initialDraft);

draft.subscribe((draft) => {
  localStorage.setItem("draft", JSON.stringify(draft));
});

store.subscribe(($store) => {
  if (!$store.identity) {
    draft.set(initialDraft);
  }
});
