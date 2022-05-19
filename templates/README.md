CSS: Instead of embedding the CDN, I ran `npx tailwindcss -i ./templates/input.css -o ./templates/output.css --minify` then embedded the small bit of CSS on each HTML file (there's only three).

It's a bit of a hack but I would rather that than rely on external connections.

Any PR that changes CSS should run `npx tailwindcss -i ./templates/input.css -o ./templates/output.css --minify` from the root of the project, then update the CSS in each HTML file.