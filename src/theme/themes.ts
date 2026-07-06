import type { GlobalThemeOverrides } from "naive-ui";

export const lightThemeOverrides: GlobalThemeOverrides = {
  common: {
    borderRadius: "10px",
    borderRadiusSmall: "6px",
    primaryColor: "#000",
    primaryColorHover: "#333",
    primaryColorPressed: "#555",
    primaryColorSuppl: "#333",
    bodyColor: "#fff",
    cardColor: "#fff",
    modalColor: "#fff",
    popoverColor: "#fff",
    textColorBase: "#000",
    textColor1: "rgba(0,0,0,0.9)",
    textColor2: "rgba(0,0,0,0.7)",
    borderColor: "rgba(0,0,0,0.09)",
    dividerColor: "rgba(0,0,0,0.06)",
    placeholderColor: "rgba(0,0,0,0.35)",
  },
};

export const darkThemeOverrides: GlobalThemeOverrides = {
  common: {
    borderRadius: "10px",
    borderRadiusSmall: "6px",
    primaryColor: "#fff",
    primaryColorHover: "#ccc",
    primaryColorPressed: "#999",
    primaryColorSuppl: "#ccc",
    bodyColor: "#121212",
    cardColor: "#1e1e1e",
    modalColor: "#1e1e1e",
    popoverColor: "#1e1e1e",
    textColorBase: "#fff",
    textColor1: "rgba(255,255,255,0.9)",
    textColor2: "rgba(255,255,255,0.7)",
    borderColor: "rgba(255,255,255,0.12)",
    dividerColor: "rgba(255,255,255,0.08)",
    placeholderColor: "rgba(255,255,255,0.35)",
  },
};
