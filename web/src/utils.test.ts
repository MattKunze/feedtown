import { assertEquals } from "https://deno.land/std@0.203.0/assert/mod.ts";

function add(a: number, b: number): number {
  return a + b;
}

Deno.test("add function", () => {
  assertEquals(add(2, 3), 5);
  assertEquals(add(-1, 1), 0);
  assertEquals(add(0, 0), 0);
});

Deno.test("sample web project test", () => {
  assertEquals(
    typeof window !== "undefined" || typeof globalThis !== "undefined",
    true,
  );
});
