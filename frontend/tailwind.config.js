/** @format */

export default {
  content: ["./src/**/*.{rs,html,css}", "./index.html"],
  darkMode: "class", // Enable class-based dark mode
  plugins: [
    function ({ addUtilities }) {
      addUtilities({
        ".scrollbar-hide": {
          /* Firefox */
          "scrollbar-width": "none",
          /* Safari and Chrome */
          "&::-webkit-scrollbar": {
            display: "none",
          },
        },
      });
    },
  ],
};
