/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}"
  ],
  theme: {
    extend: {
      colors: {
        'ct-dark-600': '#222',
        'ct-dark-200': '#e5e7eb',
        'ct-dark-100': '#f5f6f7',
        'ct-blue-600': '#2363eb',
        'ct-yellow-600': '#f9d13e',
        'ct-red-500': '#ef4444',
      },
      fontFamily: {
        Poppins: ['Poppins, sans-serif'],
      },
      fontSize: {
        sm: '0.8rem',
        base: '1rem',
        xl: '1.25rem',
        '2xl': '1.563rem',
        '3xl': '1.953rem',
        '4xl': '2.441rem',
        '5xl': '3.052rem',
      },
      container: {
        center: true,
        padding: '1rem',
        screens: {
          lg: '1125px',
          xl: '1125px',
          '2xl': '1125px',
        },
      },
    },
  },
  plugins: [],
};
