// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
import { copyFile, copyFileSync } from "./_fs_copy.ts";
import { existsSync } from "./_fs_exists.ts";

import { assert } from "../../testing/asserts.ts";
const { test } = Deno;

const destFile = "./destination.txt";

test({
  name: "[std/node/fs] copy file",
  fn: async () => {
    const err = await new Promise(async (resolve) => {
      const srouceFile = await Deno.makeTempFile();
      copyFile(srouceFile, destFile, (err: Error | undefined) => resolve(err));
      Deno.remove(srouceFile);
    });
    assert(!err);
    assert(existsSync(destFile));
    Deno.remove(destFile);
  },
});

test({
  name: "[std/node/fs] copy file sync",
  fn: () => {
    const srouceFile = Deno.makeTempFileSync();
    copyFileSync(srouceFile, destFile);
    assert(existsSync(destFile));
    Deno.removeSync(srouceFile);
    Deno.removeSync(destFile);
  },
});
