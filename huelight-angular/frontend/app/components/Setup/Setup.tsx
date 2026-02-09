import { Button } from "../ui/button"
import { CircleCheckBig } from "lucide-react"
import { SimpleInput, SimpleInputButton } from "../SimpleInput/simpleinput"

export function Setup() {
    return (
        <>
        <div className="border-solid border-2 m-2 p-2 rounded-md">
            <div className="grid gap-4 max-w-lg space-y-4">
                <h1 className="text-xl font-semibold">Setup</h1>

                <SimpleInput label="Bridge IP Address" value="" onChange={() => { }} placeholder="IP Address" />
                <SimpleInput label="API Key (Hue username)" value="" onChange={() => { }} placeholder="API Key" />

                <h1 className="text-xl font-semibold">Pairing</h1>

                <SimpleInputButton
                    label="Press link button then click 'Generate Key' to create key"
                    value="Generate Key"
                    onClick={() => { alert("Generate API Key") }}
                />

                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                    <SimpleInputButton
                        label="Status: Connected"
                        onClick={() => { alert("Test connection") }}
                        value="Test Connection"
                        labelContent={
                        // TODO: If connected, show checkmark. Otherwise, show red X.
                        <CircleCheckBig className="inline text-green-500" />}
                    />
                </div>



                <div className="pt-2">
                    <Button onClick={() => { alert("Save and Continue") }}>Save and Continue</Button>
                </div>
            </div>
        </div>
        </>
    )
}