# Slack avatar disco
A Rust app that changes your Slack avatar.

Nice to run automatically, for example every day using Systemd:

`/etc/systemd/user/slack-avatar.service`
```
[Unit]
Description=Automatically change my Slack avatar image.

[Service]
Environment=SLACK_OAUTH_TOKEN=my-secret-token
ExecStart=/stuff/slack-avatar-disco/target/release/slack-avatar-disco
```

`/etc/systemd/user/slack-avatar.timer`
```
[Unit]
Description=Automatically change my Slack avatar image.

[Timer]
OnCalendar=*-*-* 11:11:11
Persistent=true

[Install]
WantedBy=default.target
```
