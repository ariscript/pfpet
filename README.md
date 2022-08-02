# pfpet

A quick and easy way to apply filters to your friend's online avatars!
Just place their username/id in the right route and voilà, you have their avatar being petted or bonked or what have you.

Current base URL: `https://pfpet.herokuapp.com/`

## Endpoints

> Note: The `.gif` extension in the URL is required because the Discord client will only render as a static image if there is no "extension".

| URL                           | Description                                              |
| ----------------------------- | -------------------------------------------------------- |
| `/d/{filter}/{id}.gif`        | Gets a Discord user's avatar and applies a filter to it  |
| `/gh/{filter}/{username}.gif` | Gets a GitHub user's avatar and applies a filter to it   |
| `/ru/{filter}/{username}.gif` | Gets a Reddit user's avatar and applies a filter to it   |
| `/ga/{filter}/{email}.gif`    | Gets a Gravatar user's avatar and applies a filter to it |

## Filters

| Name     | Description |
| -------- | ----------- |
| nothing  | Pets        |
| `pet`    | Pets        |
| `bonk`   | Bonks       |
| `cancel` | Cancels     |

More endpoints and filters will be added soon... You can help too!

## Contributing

PRs adding new filters or image sources, or any other new features, are encouraged.
Just fork this repository, make your changes, and open a pull request.

## License

pfpet is licensed under version 3 of GNU Afferro General Public License, or at your option, any later version.
