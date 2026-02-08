import { Label } from "../ui/label"
import { Input } from "../ui/input"
import { Field, FieldLabel } from "../ui/field"
import { Button } from "../ui/button"
import { CircleCheckBig } from "lucide-react"
import { SimpleInput, SimpleInputButton } from "../SimpleInput/simpleinput"

export function Setup() {
    return (
        <div className="border-solid border-2 m-2 p-2 rounded-md">
            <div className="grid gap-4 max-w-xl">
                <h1 className="text-xl font-semibold">Setup</h1>

                <SimpleInput label="Bridge IP Address" value="" onChange={() => { }} placeholder="IP Address" />
                <SimpleInput label="API Key (Hue username)" value="" onChange={() => { }} placeholder="API Key" />

                <h2 className="pt-4 font-semibold">Pairing</h2>

                <SimpleInputButton
                    label="Press link button then click 'Generate Key' to create key"
                    value=""
                    onChange={() => { }}
                    placeholder="Generate Key"
                />

                <SimpleInputButton
                    label="Status: Connected"
                    value=""
                    onChange={() => { }}
                    placeholder="Test Connection"
                />

                <div className="pt-2">
                    <Button>Save and Continue</Button>
                </div>
            </div>
        </div>
    )
}