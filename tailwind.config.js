const colors = require('tailwindcss/colors')
//const daisyui = require('daisyui')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ['*.html', './app/src/**/*.rs'],
  },
  theme: {
    container: {
      center: true,
      screens: {
        "2xl": "1400px",
      },
    },
    extend: {
      colors: {
        // picked from: https://m2.material.io/inline-tools/color/
        //primary: "#8f2c45", // a purple
        primary: "#f10f7c",
        secondary: "#00f454",
        //secondary: "#2c8f76", // a teal
        //accent: "#8f452c", // a brown
        paper: "#FFFFFF",
        // material dark mode background color
        "dark-gray": "#121212",
      },
    },
  },
  plugins: [],
}
