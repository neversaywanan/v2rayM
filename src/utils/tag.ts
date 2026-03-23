export function sanitizeTag(name: string): string {
  let raw = "";
  for (const c of name) {
    // Keep Unicode letters/numbers (matches Rust `char::is_alphanumeric` behavior)
    // plus "_", "-", ".".
    if (c === "_" || c === "-" || c === "." || /^[\p{L}\p{N}]$/u.test(c)) {
      raw += c;
    } else {
      raw += "_";
    }
  }

  let tag = "";
  let prevUnder = false;
  for (const c of raw) {
    if (c === "_") {
      if (!prevUnder && tag.length > 0) tag += "_";
      prevUnder = true;
    } else {
      tag += c;
      prevUnder = false;
    }
  }

  tag = tag.replace(/^_+|_+$/g, "");
  return tag || "proxy_node";
}
