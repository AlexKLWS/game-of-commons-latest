import { DnaSource } from "@holochain/client";
import { pause, runScenario } from "@holochain/tryorama";
import pkg from "tape-promise/tape";
const { test } = pkg;

import { dna0Dna } from "../../utils";

export default () =>
  test("game_logic tests", async (t) => {
    await runScenario(async (scenario) => {
      const dnas: DnaSource[] = [{ path: dna0Dna }];
      // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
      // note that the first argument to players is just an array conductor configs that that will
      // be used to spin up the conductor processes which are returned in a matching array.

      const [alice_happ, bob_happ] = await scenario.addPlayersWithHapps([
        dnas,
        dnas,
      ]);

      await scenario.shareAllAgents();

      const alice = alice_happ.cells[0];
      const bob = bob_happ.cells[0];

      const ZOME_NAME = "zome_0";
      const GAME_CODE = "ABCDE";

      // Alice creates a game code
      const codeHash = await alice.callZome({
        zome_name: ZOME_NAME,
        fn_name: "create_game_code_anchor",
        payload: GAME_CODE,
      });
      console.log("Alice created the game code: ", codeHash);
      t.ok(codeHash);

      await pause(50);

      // Alice joins the game with this code
      const joinHashAlice = await alice.callZome({
        zome_name: ZOME_NAME,
        fn_name: "join_game_with_code",
        payload: {
          gamecode: GAME_CODE,
          nickname: "Alice",
        },
      });
      console.log("Alice joined the game: ", joinHashAlice);
      t.ok(joinHashAlice);

      // Bob joins the game with this code
      const joinHashBob = await bob.callZome({
        zome_name: ZOME_NAME,
        fn_name: "join_game_with_code",
        payload: {
          gamecode: GAME_CODE,
          nickname: "Bob",
        },
      });
      console.log("Bob joined the game: ", joinHashBob);
      t.ok(joinHashBob);

      await pause(50);

      let list_of_players: any = await alice.callZome({
        zome_name: ZOME_NAME,
        fn_name: "get_players_for_game_code",
        payload: GAME_CODE,
      });
      console.log("List of players in the game: ", list_of_players);
      t.ok(list_of_players);
      // Verify that there actually 2 players in the game: no more, no less
      t.ok(list_of_players.length == 2);

      //Alice starts a new game (session) with the game code
      let session_header_hash = await alice.callZome({
        zome_name: ZOME_NAME,
        fn_name: "start_game_session_with_code",
        payload: GAME_CODE,
      });
      console.log("Alice created new game session:", session_header_hash);
      t.ok(session_header_hash);

      await pause(150);

      let alice_owned_games: any = await alice.callZome({
        zome_name: ZOME_NAME,
        fn_name: "get_my_owned_sessions",
        payload: null,
      });
      console.log("Verify that Alice's owned games is 1");
      t.ok(alice_owned_games.length == 1);

      let bob_owned_games: any = await bob.callZome({
        zome_name: ZOME_NAME,
        fn_name: "get_my_owned_sessions",
        payload: null,
      });
      console.log("Verify that Bob's owned games is 0");
      t.ok(bob_owned_games.length == 0);

      let all_codes: string[] = await alice.callZome({
        zome_name: ZOME_NAME,
        fn_name: "get_all_game_codes",
        payload: null,
      });
      t.ok(all_codes);
      console.log(
        "???? ~ file: game_logic.ts ~ line 108 ~ awaitrunScenario ~ all_codes",
        all_codes
      );
    });
  });
