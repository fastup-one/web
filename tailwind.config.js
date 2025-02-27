/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: 'jit',
  content: [
    "./src/**/*.rs",
    "./src/**/*.html",
    "./input.css"
  ],
  theme: {
    extend: {
      animation: {
<<<<<<< HEAD
        'float': 'float 6s ease-in-out infinite',
        'pulse': 'pulse 3s ease-in-out infinite',
        'fade-in': 'fade-in 0.8s ease-out forwards',
        'gradient-shift': 'gradient-shift 8s ease infinite',
      },
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-15px)' },
        },
        pulse: {
          '0%, 100%': { opacity: 0.7 },
          '50%': { opacity: 0.3 },
        },
        'fade-in': {
          from: { opacity: 0, transform: 'translateY(20px)' },
          to: { opacity: 1, transform: 'translateY(0)' },
        },
        'gradient-shift': {
          '0%': { backgroundPosition: '0% 50%' },
          '50%': { backgroundPosition: '100% 50%' },
          '100%': { backgroundPosition: '0% 50%' },
        },
      },
      colors: {
        slate: {
          950: '#0B1120',
        },
        indigo: {
          950: '#1A1346',
        },
      },
      boxShadow: {
        'glow': '0 4px 20px -2px rgba(96, 165, 250, 0.25)',
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
      },
    },
  },
  plugins: [],
}
=======
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
>>>>>>> refactor-001
