/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        heading: ['Bricolage Grotesque', 'sans-serif'],
        body: ['Inter', 'sans-serif'],
      },
      colors: {
        glass: {
          base: 'rgba(255, 255, 255, 0.05)',
          elevated: 'rgba(255, 255, 255, 0.08)',
          hover: 'rgba(255, 255, 255, 0.12)',
        },
      },
      backdropBlur: {
        glass: '20px',
        'glass-xl': '24px',
      },
    },
  },
  plugins: [],
}
