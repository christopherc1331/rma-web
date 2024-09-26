use crate::{
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    input::{Input, InputType},
};
use leptos::{component, view, IntoView};
use leptos_material::components::{icon::Icon, iconbutton::IconButton};

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen">
        <header class="px-4 lg:px-6 h-14 flex items-center">
            <a class="flex items-center justify-center" href="#">
            <Icon name="flare"/>
            <span class="font-bold">{"TattooRatings"}</span>
            </a>
            <nav class="ml-auto flex gap-4 sm:gap-6">
                <a class="text-sm font-medium hover:underline underline-offset-4" href="#">
                    {"Home"}
                </a>
                <a class="text-sm font-medium hover:underline underline-offset-4" href="#">
                    {"Artists"}
                </a>
                <a class="text-sm font-medium hover:underline underline-offset-4" href="#">
                    {"Reviews"}
                </a>
                <a class="text-sm font-medium hover:underline underline-offset-4" href="#">
                    {"About"}
                </a>
            </nav>
        </header>
        <main class="flex-1">
            <section class="w-full py-12 md:py-24 lg:py-32 xl:py-48 bg-black text-white">
            <div class="container px-4 md:px-6">
                <div class="flex flex-col items-center space-y-4 text-center">
                <div class="space-y-2">
                    <h1 class="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none">
                    {"Find Your Perfect Tattoo Artist"}
                    </h1>
                    <p class="mx-auto max-w-[700px] text-gray-300 md:text-xl">
                    {"Discover, rate, and review tattoo artists based on customer service and tattoo quality. Share your
                    experience and help others find their ideal artist."}
                    </p>
                </div>
                <div class="w-full max-w-sm space-y-2">
                    <form class="flex space-x-2">
                        <Input
                            class="max-w-lg flex-1 bg-white text-black"
                            placeholder="Search for artists or styles"/>
                        <Button size=ButtonSize::Icon>
                            <div class="h-4 w-4 mr-2">
                                <Icon name="search"/>
                            </div>
                            {"Search"}
                        </Button>
                    </form>
                </div>
                </div>
            </div>
            </section>
            <section class="w-full py-12 md:py-24 lg:py-32 bg-gray-100">
            <div class="container px-4 md:px-6">
                <h2 class="text-3xl font-bold tracking-tighter sm:text-5xl text-center mb-8">Featured Reviews</h2>
                <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                <Card>
                    <CardContent class="p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-2xl font-bold">John Doe</h3>
                        <div class="flex">
                            <Icon name="star"/>
                            <Icon name="star"/>
                            <Icon name="star"/>
                            <Icon name="star"/>
                            <Icon name="star"/>
                        </div>
                    </div>
                    <img
                        alt="Tattoo by John Doe"
                        class="w-full h-40 object-cover rounded-md mb-4"
                        height="160"
                        src="/placeholder.svg?height=160&width=300"
                        style="aspect-ratio: 300/160; object-fit: cover;"
                        width="300"
                    />
                    <p class="text-gray-600 mb-2">Customer Service: 5/5</p>
                    <p class="text-gray-600 mb-4">Tattoo Quality: 5/5</p>
                    <p class="text-sm text-gray-600">
                        "Amazing experience! John's attention to detail and friendly demeanor made getting my first tattoo a
                        breeze. Highly recommended!"
                    </p>
                    </CardContent>
                </Card>
                <Card>
                    <CardContent class="p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-2xl font-bold">Jane Smith</h3>
                        <div class="flex">
                            <Icon name="star"/>
                            <Icon name="star"/>
                            <Icon name="star"/>
                            <Icon name="star"/>
                            <Icon name="star-half-full"/>
                        </div>
                    </div>
                    <img
                        alt="Tattoo by Jane Smith"
                        class="w-full h-40 object-cover rounded-md mb-4"
                        height="160"
                        src="/placeholder.svg?height=160&width=300"
                        style="aspect-ratio: 300/160; object-fit: cover;"
                        width="300"
                    />
                    <p class="text-gray-600 mb-2">Customer Service: 4/5</p>
                    <p class="text-gray-600 mb-4">Tattoo Quality: 5/5</p>
                    <p class="text-sm text-gray-600">
                        "Jane's artistic skills are top-notch! The final piece exceeded my expectations. Only giving 4 stars
                        for customer service due to a slight delay in scheduling."
                    </p>
                    </CardContent>
                </Card>
                <Card>
                    <CardContent class="p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-2xl font-bold">Mike Johnson</h3>
                        <div class="flex">
                            <Icon name="star"/>
                            <Icon name="star"/>
                            <Icon name="star"/>
                            <Icon name="star"/>
                            <Icon name="star"/>
                        </div>
                    </div>
                    <img
                        alt="Tattoo by Mike Johnson"
                        class="w-full h-40 object-cover rounded-md mb-4"
                        height="160"
                        src="/placeholder.svg?height=160&width=300"
                        style="aspect-ratio: 300/160; object-fit: cover;"
                        width="300"
                    />
                    <p class="text-gray-600 mb-2">Customer Service: 5/5</p>
                    <p class="text-gray-600 mb-4">Tattoo Quality: 5/5</p>
                    <p class="text-sm text-gray-600">
                        "Mike's creativity and professionalism are unmatched. He took my vague idea and turned it into a
                        masterpiece. The studio was clean and welcoming too!"
                    </p>
                    </CardContent>
                </Card>
                </div>
            </div>
            </section>
            <section class="w-full py-12 md:py-24 lg:py-32">
            <div class="container px-4 md:px-6">
                <h2 class="text-3xl font-bold tracking-tighter sm:text-5xl text-center mb-8">How It Works</h2>
                <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
                <div class="flex flex-col items-center text-center">
                    <div class="h-12 w-12 mb-4 text-primary">
                        <Button size=ButtonSize::Icon label="Search" >
                            <Icon name="search"/>
                        </Button>
                    </div>
                    <h3 class="text-xl font-bold mb-2">Find Artists</h3>
                    <p class="text-gray-600">Search for tattoo artists in your area or by style.</p>
                </div>
                <div class="flex flex-col items-center text-center">
                    <div class="h-12 w-12 mb-4 text-primary" >
                        <Icon name="star"/>
                    </div>
                    <h3 class="text-xl font-bold mb-2">Rate & Review</h3>
                    <p class="text-gray-600">{"Share your experience and rate the artist's work and service."}</p>
                </div>
                <div class="flex flex-col items-center text-center">
                    <div class="h-12 w-12 mb-4 text-primary">
                        <Icon name="message" />
                    </div>
                    <h3 class="text-xl font-bold mb-2">Help Others</h3>
                    <p class="text-gray-600">Your reviews help others find their perfect tattoo artist.</p>
                </div>
                </div>
            </div>
            </section>
            <section class="w-full py-12 md:py-24 lg:py-32 bg-black text-white">
            <div class="container px-4 md:px-6">
                <div class="flex flex-col items-center space-y-4 text-center">
                <div class="space-y-2">
                    <h2 class="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl">
                    Ready to Share Your Experience?
                    </h2>
                    <p class="mx-auto max-w-[700px] text-gray-300 md:text-xl">
                    Your review can help others find their perfect tattoo artist. Start sharing now!
                    </p>
                </div>
                <Button class="bg-white text-black hover:bg-gray-200" size=ButtonSize::Large>
                    Write a Review
                </Button>
                </div>
            </div>
            </section>
        </main>
        <footer class="flex flex-col gap-2 sm:flex-row py-6 w-full shrink-0 items-center px-4 md:px-6 border-t">
            <p class="text-xs text-gray-500">{"© 2023 TattooRatings. All rights reserved."}</p>
            <nav class="sm:ml-auto flex gap-4 sm:gap-6">
            <a class="text-xs hover:underline underline-offset-4" href="#">
                Terms of Service
            </a>
            <a class="text-xs hover:underline underline-offset-4" href="#">
                Privacy
            </a>
            </nav>
        </footer>
        </div>
    }
}
