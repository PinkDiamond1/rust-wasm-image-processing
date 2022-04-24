<template>
    <div>
        <h1>{{ greeting }}</h1>
        <div class="container mt-10">
            <div class="card bg-white">
                <input @change="handleImage" class="custom-input" type="file" accept="image/*" />
                <img style="" :src="image" alt="" />
                <img style="" :src="imageEdit" alt="" />
            </div>
        </div>
    </div>
</template>

<script>
    import init, * as img from 'image_process';
    import { FromByteArrayToBase64 } from '../utils/conversion';

    await init();

    // const base64_arraybuffer = async (data) => {
    // // Use a FileReader to generate a base64 data URI
    //     const base64url = await new Promise((r) => {
    //         const reader = new FileReader()
    //         reader.onload = () => r(reader.result)
    //         reader.readAsDataURL(new Blob([data]))
    //     })

    //     /*
    //     The result looks like
    //     "data:application/octet-stream;base64,<your base64 data>",
    //     so we split off the beginning:
    //     */
    //     return base64url.split(",", 2)[1]
    // }

    export default {
        data() {
            return {
                image: '',
                imageEdit: '',
                greeting: ''
            }
        },
        methods: {
            handleImage(e) {
                // console.log(img.hello());
                const selectedImage = e.target.files[0];
                this.createBase64Image(selectedImage);
            },
            createBase64Image(fileObject) {
                const reader = new FileReader();

                reader.onload = (e) => {
                    this.image = e.target.result;
                    // this.greeting = img.hello();
                   this.onImageUploaded();
                   //console.log(img.test_call(this.image.substring(1, 10)));
                };

                reader.readAsDataURL(fileObject);
            },
            async onImageUploaded() {
                // this.imageEdit = img.test(this.image);
                // this.imageEdit = img.perform_processing(this.image);
                let array = img.perform_processing(this.image);
                console.log(array);
                var b64 = await FromByteArrayToBase64(array);
                console.log(b64);
                this.imageEdit = "data:image/jpeg;base64," + b64;
            }
        }
    }
</script>

<style scoped>
* {
    font-family: Arial;
}

body {
    background: #d8dddb;
}

.container {
    display:flex;
    justify-content: center;
}
.mt-10 {
    margin-top:10rem;
}

.bg-white {
    background: white;
}

.card {
    height: 10rem;
    width: 20rem;
    border-radius: 10px;
    padding: 20px;
    text-align: center;
}

img {
    width: 17rem;
}

</style>