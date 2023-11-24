const colors = require('tailwindcss/colors')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ['*.html', './src/**/*.rs'],
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
        primary: "#8f2c45", // a purple
        secondary: "#2c8f76", // a teal
        accent: "#8f452c", // a brown
      },
    },
  },
  plugins: [],
}

