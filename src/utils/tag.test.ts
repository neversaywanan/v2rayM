import { describe, expect, test } from "vitest";

import { sanitizeTag } from "./tag";

describe("sanitizeTag", () => {
  test("keeps unicode alphanumeric like circled digits", () => {
    expect(sanitizeTag("①香港")).toBe("①香港");
  });

  test("replaces spaces and special chars with underscore", () => {
    expect(sanitizeTag("A B/C"))
      .toBe("A_B_C");
  });

  test("trims and collapses underscores", () => {
    expect(sanitizeTag("__A  B__"))
      .toBe("A_B");
  });
});
