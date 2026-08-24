import {$} from "bun";
// please refer https://bun.sh/docs/runtime/shell for Bun's shell scripting API

export async function hello() {
    await $`echo Hello World!`;
}
export async function list_js() {
    await $`ls *.js`;
}
