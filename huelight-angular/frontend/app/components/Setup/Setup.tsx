import { Label } from "../ui/label"
import { Input } from "../ui/input"
import { Field, FieldLabel } from "../ui/field"
import { Button } from "../ui/button"
import { CircleCheckBig } from "lucide-react"
import { SimpleInput, SimpleInputButton } from "../SimpleInput/simpleinput"

export function Setup() {
    return (
        <div className="border-solid border-2 m-2 p-2 rounded-md grid gap-2">
            <h1>Setup</h1>
            <SimpleInput label="Bridge IP Address" value="" onChange={() => {}} placeholder="IP Address" />
            <SimpleInput label="API Key (Hue username)" value="" onChange={() => {}} placeholder="API Key" />
            
            <br />
            <h1>-- OR --</h1>
            <br />
            
            <SimpleInputButton label="Pairing: Press link button then click 'Generate Key' to create key" value="" onChange={() => {}} placeholder="Generate Key" />
            <SimpleInputButton label="Status: Connected" value="" onChange={() => {}} placeholder="Test Connection" />
            
            <div className="m-1">
                <Button>Save and Continue</Button>
            </div>

        </div>
    )
}