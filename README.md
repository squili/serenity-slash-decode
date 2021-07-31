# Serenity Slash Decode
After banging my head against a wall trying to get
[slash commands](https://discord.com/developers/docs/interactions/slash-commands) to make sense in
[Serenity](https://docs.rs/serenity/latest/serenity/), in addition to the impending
[Discord intent changes](https://support-dev.discord.com/hc/en-us/articles/4404772028055), I decided to make a library
to help with easier parsing of command arguments. Inspired by [Clap's](https://clap.rs/) `ArgMatches`, you're able to
parse arguments in one call, match which function to execute for which interaction, and extract values from the
arguments. You're even able to use `?`s to exit early from your command's individual function.