# indirect11
Hand written bindings for DirectX 11 made with Microsoft's com crate. For the learning experience.

**NOTE** Currently not usable. Can't even do screen refresh.

##Points to Remember
NOTE that com crate is still in development and will change a lot. No need to rush. MS may make its own bindings perhaps anyways. Also a code gen is smarter than hand work.

* Com interface GUIDs are in hex format of 8-4-4-4-12 characters with a total of 32 characters.
* The terminology "COM interface" is not the same as "Rust trait" or "C# interface".
* Most interfaces derive from IUnknown directly or indirectly.
* Client side COM is the code that consumes COM based APIs. Server side COM creates COM based API. Using DirectX is client side.
* Client side COM is much simpler than Server side COM.
* COM ain't safe by default. But it can mimic Rust grade safe behavior with programmer effort.
* Methods/fields can have any name, but they most match the order and number on the VTable. The signature/type must match, too.
* The com_interface attribute does inform about usage of its interface below.Lack of unsafe functions, of '&self', lack of a specific function name among other, they are all reported and cause a build fail.
* CLSID are used by create_instance. Some of them can be found in crate dxguid-sys.

##Questions
* Are structs just defined like normal Rust structs?
* Are constants defined like normal Rust constants? Guess, yes.
* Are enums defined like normal Rust enums? Guess, yes.

## License

* Rust crate indirect11 is not affiliated with Microsoft Corporation.
* MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

[LICENSE-MIT]: LICENSE-MIT