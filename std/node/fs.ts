// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.

export { access, accessSync } from "./_fs/_fs_access.ts";
export { appendFile, appendFileSync } from "./_fs/_fs_appendFile.ts";
export { chmod, chmodSync } from "./_fs/_fs_chmod.ts";
export { chown, chownSync } from "./_fs/_fs_chown.ts";
export { close, closeSync } from "./_fs/_fs_close.ts";
import * as constants from "./_fs/_fs_constants.ts";
export { constants };
export { readFile, readFileSync } from "./_fs/_fs_readFile.ts";
export { readlink, readlinkSync } from "./_fs/_fs_readlink.ts";
export { exists, existsSync } from "./_fs/_fs_exists.ts";
export { mkdir, mkdirSync } from "./_fs/_fs_mkdir.ts";
export { copyFile, copyFileSync } from "./_fs/_fs_copy.ts";
export { writeFile, writeFileSync } from "./_fs/_fs_writeFile.ts";
import * as promises from "./_fs/promises/mod.ts";
export { promises }