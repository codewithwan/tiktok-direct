export const string = (value) => (value == null ? "" : String(value));

export const number = (value) => {
  if (value == null || value === "") return null;
  const parsed = Number(value);
  return Number.isNaN(parsed) ? null : parsed;
};

export function pickUrl(value) {
  if (typeof value === "string") return value;
  if (Array.isArray(value)) return value.find((item) => typeof item === "string") || "";
  if (value) return pickUrl(value.urlList || value.url_list || value.urls);
  return "";
}

export function unique(values) {
  return [...new Set(values.filter((value) => typeof value === "string" && value.startsWith("http")))];
}
