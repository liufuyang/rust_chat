# Temp readme

For dealing with https://github.com/tokio-rs/tokio/issues/3031

The basic sketch is going to be:
1. Add an `into_inner` method on `PollEvented`. Make sure to deregister the stream from Tokio.
2. Convert the `mio` tcp stream into an std one. You may have to go through mio's `IntoRawFd` to do so.

The reason there are two different tcp streams is that Tokio has an event loop (provided by the mio crate), 
which IO resources need to register to. The std tcp stream does not register with it, so you need a different type

You can think of mio as part of Tokio
Well there are several parts to the event loop. Mio specifically deals with talking to the OS. It's basically a collection of IO resources that allows you to ask the OS whether any of them has had an event
So tokio uses mio to figure out which tasks has an io event, then wakes those tasks up