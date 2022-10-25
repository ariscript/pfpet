# pfpet

A quick and easy way to apply filters to your friend's online avatars!
Just place their username/id in the right route and voilà, you have their avatar being petted or bonked or what have you.

Current base URL: `https://pfpet.msterling.dev/`

## Endpoints

> Note: The `.gif` extension in the URL is required because the Discord client will only render as a static image if there is no "extension".

| URL                       | Description                               |
| ------------------------- | ----------------------------------------- |
| `/d/{id}.gif`             | Gets a Discord user's avatar and pets it  |
| `/d/bonk/{id}.gif`        | Gets a Discord user's avatar and bonks it |
| `/gh/{username}.gif`      | Gets a GitHub user's avatar and pets it   |
| `/gh/bonk/{username}.gif` | Gets a GitHub user's avatar and bonks it  |
| `/ru/{username}.gif`      | Gets a Reddit user's avatar and pets it   |
| `/ru/bonk/{username}.gif` | Gets a Reddit user's avatar and bonks it  |
| `/ga/{email}.gif`         | Gets a Reddit user's avatar and pets it   |
| `/ga/bonk/{email}.gif`    | Gets a Reddit user's avatar and bonks it  |

More endpoints will be added soon... You can help too!

## Contributing

PRs adding new filters or image sources, or any other new features, are encouraged.
Just fork this repository, make your changes, and open a pull request.

## License

pfpet is licensed under version 3 of GNU Afferro General Public License, or at your option, any later version.
