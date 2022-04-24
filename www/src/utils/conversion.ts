const FromByteArrayToBase64 = async (data : Uint8Array) : Promise<string> => {
        const base64url: string = await new Promise((r) => {
            const reader = new FileReader()
            reader.onload = () => r(reader.result.toString())
            reader.readAsDataURL(new Blob([data]))
        })

        /*
        The result looks like
        "data:application/octet-stream;base64,<your base64 data>",
        so we split off the beginning:
        */
        return base64url.split(",", 2)[1]
    }

export { FromByteArrayToBase64 }