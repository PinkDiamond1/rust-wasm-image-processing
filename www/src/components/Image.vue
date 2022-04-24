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
                try {
                    let x = img.throw_error();
                } catch(error) {
                    console.log("Error received from Rust : " + error)
                }


                // this.imageEdit = img.perform_processing(this.image);
                try {
                    let params = new img.ImageParameters();
                    params.brighten = 10;
                    params.blur = 0;
                    params.hue = 100;
                    params.grayscale = true;
                    params.constrast = 0;
                    // let params = img.ImageParameters().new_img_params(0, 0.0, 0, false, 0.0);
                    let image_result = img.perform_processing(this.image, params);
                    this.imageEdit = "data:image/jpeg;base64," + image_result.to_base64();
                } catch(error) {
                    console.log(error)
                }
                // console.log(array);
                // var b64 = img.b(array);
                // var b64 = await FromByteArrayToBase64(array);
                // console.log(b64);
                // this.imageEdit = "data:image/jpeg;base64," + b64;
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