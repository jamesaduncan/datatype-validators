# Usage:

    ```javascript
        import { Validator } from "validator.mjs"

        // validator wasm files have one function, validate, and take a single argument
        // which is the data that should be validated.
        const tvalidator = new Validator( "http://foo.bar.com/text-validator.wasm" );
        tvalidator.validate( "foo" );

        // calls the validate() function in the WASM file, with the argument: { value: "foo" }

        const ovalidator = new Validator("http://foo.bar.com/object-validator.wasm");
        ovalidator.validate( { foo: "bar" } );
        // calls the validate() function in the WASM file with the argument: { value: { foo: "bar" } }

        // alternately, you can assert
        try {
            ovalidator.assert( "foo" )
        } catch(e) {
            // will throw a ValidationError, because ovalidator checks for the presence of an object
        }
    ```


