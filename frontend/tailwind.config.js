/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.html",
    "./src/**/*.rs",
    "./src/**/*.css",
    "./index.html",
  ],
  theme: {
    extend: {
        colors: {
            primary_dark:"#5F5D7D",
            primary:"#8785A2",
            primary_light:"#B3B1C6",
            secondary_dark:"#E5E5E5",
            secondary:"#F6F6F6",
            secondary_light:"#F9F9F9",
            accent_dark:"#FFA8A8",
            accent:"#FFC7C7",
            accent_light:"#FFE2E2",
        },
        screens: {
            '2xl': '1536px',
            '3xl': '1780px',
            '4xl': '1920px',
        }
    },
  },
  plugins: [],
}

