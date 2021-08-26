# Rubicon
## Scouting the frontlines of your platform

Rubicon is a tool for querying common deployment platforms for information about the current going-on's.

For example, the Netlify integration allows you to list sites (and eventually, we'll get more information about those sites).

Rubicon uses a config file named .rubicon. It's a .env file so do with that what you will. However, its important to store any access tokens as environment variables or in that .rubicon file (home directory is fine). These get parsed into the rubicon runtime config and passed to the platform-specific commands.