// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.

import { assertEquals } from "../../testing/asserts.ts";
import { exists, existsSync } from "./_fs_exists.ts";
const { test } = Deno;

test(async function existsFile() {
  const availableFile = await new Promise((resolve) => {
    const tmpFilePath = Deno.makeTempFileSync();
    exists(tmpFilePath, (exists: boolean) => {
      Deno.removeSync(tmpFilePath);
      resolve(exists);
    });
  });
  const notAvailableFile = await new Promise((resolve) => {
    exists("./notAvailable.txt", (exists: boolean) => resolve(exists));
  });
  assertEquals(availableFile, true);
  assertEquals(notAvailableFile, false);
});

test(function existsSyncFile() {
  const tmpFilePath = Deno.makeTempFileSync();
  assertEquals(existsSync(tmpFilePath), true);
  Deno.removeSync(tmpFilePath);
  assertEquals(existsSync("./notAvailable.txt"), false);
});
