/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.{html,rs}'],
    theme: {
        extend: {
            colors: {
                accent: 'var(--accent-color)',
                red: '#fd8c7b', // 'oklch(76% 0.14 30)',
                yellow: '#d0ae32', // 'oklch(76% 0.14 93)',
                green: '#7fc76f', // 'oklch(76% 0.14 140)',
                blue: '#30c0f8', // 'oklch(76% 0.14 230)',
                purple: '#bf9bfc', // 'oklch(76% 0.14 300)',
                pink: '#eb8ccd', // 'oklch(76% 0.14 340)',
            },
        },
    },
    plugins: [],
}

