export function deepEqual(a: any, b: any) {
  const typeofA = typeof a;
  const typeofB = typeof b;
  if (typeofA !== typeofB) {
    return false;
  }
  if (
    typeofA === "bigint" ||
    typeofA === "boolean" ||
    typeofA === "string" ||
    typeofA === "number" ||
    typeofA === "undefined" ||
    typeofA === "symbol"
  ) {
    if (a === b) {
      return true;
    }
    if (typeofA === "number" && isNaN(a) && isNaN(b)) {
      return true;
    }
    return false;
  }

  if (typeofA === "object") {
    if (a === null || b === null) {
      return a === b;
    }
    if (Array.isArray(a) || Array.isArray(b)) {
      if (!Array.isArray(a) || !Array.isArray(b)) {
        return false;
      }
      // Compare arrays
      if (a.length !== b.length) {
        return false;
      }
      for (let i = 0; i < a.length; i++) {
        if (!deepEqual(a[i], b[i])) {
          return false;
        }
      }
      return true;
    }

    // Compare objects
    const keysA = new Set(Object.keys(a));
    const keysB = new Set(Object.keys(b));
    if (keysA.size !== keysB.size) {
      return false;
    }
    for (const kA of keysA) {
      if (!keysB.has(kA)) {
        return false;
      }
    }
    for (const kB of keysB) {
      if (!keysA.has(kB)) {
        return false;
      }
    }
    for (const key of keysA) {
      if (!deepEqual(a[key], b[key])) {
        return false;
      }
    }
    return true;
  }
  return false;
}
