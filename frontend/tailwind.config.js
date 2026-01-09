/**
 * @format
 * @type {import('tailwindcss').Config}
 */

module.exports = {
  content: ["./src/**/*.{rs,html,css}", "./index.html"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        // Theme Colors
        primary: {
          dark: "var(--color-primary-dark)",
          medium: "var(--color-primary-medium)",
        },
        accent: "var(--color-accent)",
        theme: {
          primary: {
            50: "var(--color-bg-primary)",
            100: "var(--color-bg-secondary)",
            200: "var(--color-bg-tertiary)",
            500: "var(--color-accent)",
            600: "var(--color-primary-medium)",
            700: "var(--color-primary-dark)",
            900: "var(--color-text-primary)",
          },
          success: {
            400: "var(--color-accent)",
            600: "var(--color-accent)",
          },
          error: {
            400: "var(--color-primary-dark)",
            500: "var(--color-primary-dark)",
            600: "var(--color-primary-dark)",
          },
          warning: {
            400: "var(--color-light)",
            600: "var(--color-light)",
          },
          surface: {
            50: "var(--color-bg-primary)",
            100: "var(--color-bg-secondary)",
            200: "var(--color-bg-tertiary)",
            700: "var(--color-bg-secondary)",
            800: "var(--color-bg-tertiary)",
            900: "var(--color-bg-primary)",
          },
          text: {
            primary: "var(--color-text-primary)",
            secondary: "var(--color-text-secondary)",
            accent: "var(--color-text-accent)",
            100: "var(--color-text-primary)",
            200: "var(--color-text-secondary)",
            300: "var(--color-text-secondary)",
            400: "var(--color-text-secondary)",
            500: "var(--color-text-secondary)",
            600: "var(--color-text-secondary)",
            700: "var(--color-text-primary)",
            800: "var(--color-text-primary)",
            900: "var(--color-text-primary)",
          },
          border: {
            200: "var(--color-bg-tertiary)",
            300: "var(--color-bg-tertiary)",
            600: "var(--color-bg-secondary)",
            700: "var(--color-bg-tertiary)",
          },
          bg: {
            primary: "var(--color-bg-primary)",
            secondary: "var(--color-bg-secondary)",
            tertiary: "var(--color-bg-tertiary)",
          },
        },

        // Direct color access
        dark: "var(--color-primary-dark)",
        medium: "var(--color-primary-medium)",
        teal: "var(--color-accent)",
        light: "var(--color-light)",
      },
    },
  },
  plugins: [],
};
