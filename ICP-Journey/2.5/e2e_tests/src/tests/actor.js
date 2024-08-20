import { Actor, HttpAgent } from "@dfinity/agent";
import fetch from "isomorphic-fetch";
import canisterIds from ".dfx/local/canister_ids.json";
import { idlFactory } from "../declarations/e2e_tests_backend/e2e_tests_backend.did.js";

export const createActor = async (canisterId, options) => {
  const agent = new HttpAgent({ ...options?.agentOptions });
  await agent.fetchRootKey();

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
    ...options?.actorOptions,
  });
};

export const e2e_tests_backendCanister = canisterIds.e2e_tests_backend.local;

export const e2e_tests_backend = await createActor(e2e_tests_backendCanister, {
  agentOptions: { host: "http://127.0.0.1:4943", fetch },
});