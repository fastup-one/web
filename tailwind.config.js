/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      animation: {
        'gradient-xy': 'gradient-xy 15s ease infinite',
        'float': 'float 6s ease-in-out infinite',
      },
      backgroundSize: {
        'auto': 'auto',
        'cover': 'cover',
        'contain': 'contain',
        '200%': '200%',
      },
      backdropBlur: {
        xs: '2px',
      },
      colors: {
        'nebula-blue': '#0ea5e9',
        'nebula-purple': '#7c3aed',
      },
      boxShadow: {
        glow: '0 0 15px 5px rgba(59, 130, 246, 0.3)',
      }
    },
  },
  plugins: [],
};