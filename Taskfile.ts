import {$} from "bun";

export async function hello() {
    await $`echo Hello World!`;
}
export async function list_js() {
    await $`ls *.js`;
}
