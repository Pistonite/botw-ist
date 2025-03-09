async function boot() {
    const root = document.getElementById("-root-") as HTMLDivElement;
    const bootPromises = [probeAndRegisterAssetLocation(), initI18n()];
    if (isLessProductive) {
        // window.setStatus
        // await new Promise<void>((resolve) => {
        //     const button = document.createElement('button');
        //     button.innerText = 'fullscreen' + window.innerWidth;
        //     button.onclick = async () => {
        //     // document.body.style.height = 'calc ( 100vh + 1px )';
        //     // document.body.style.overflow = 'visible';
        //     // root.style.height = 'calc ( 100vh + 1px )';
        //     // window.scrollTo(0, 100);
        //         await document.body.requestFullscreen({
        //             navigationUI: "hide"});
        //         resolve();
        //     };
        //     root.appendChild(button);
        // });
        initNarrow({
            threshold: 800,
            override: (narrow) => {
                if (window.innerWidth < window.innerHeight) {
                    return true;
                }
                if (narrow && window.innerHeight < window.innerWidth) {
                    return false;
                }
                return narrow;
            },
        });
    } else {
        initNarrow({
            threshold: 800,
        });
    }
    initDark({
        persist: false,
    });
    initExtensionManager();
    const queryClient = new QueryClient();

    const runtime = await initRuntime();
    const app = createExtensionAppHost(runtime);

    await Promise.all(bootPromises);

    addLocaleSubscriber(() => {
        const title = translateUI("title");
        // fallback in case translation failed to load
        if (title === "title") {
            document.title = "IST Simulator";
        } else {
            document.title = title;
        }
    }, true);

    createRoot(root).render(
        <StrictMode>
            <ExtensionAppContext.Provider value={app}>
                <QueryClientProvider client={queryClient}>
                    <ThemeProvider>
                        <ItemTooltipProvider
                            backgroundUrl={getSheikaBackgroundUrl()}
                        >
                            <App />
                        </ItemTooltipProvider>
                    </ThemeProvider>
                </QueryClientProvider>
            </ExtensionAppContext.Provider>
        </StrictMode>,
    );
}
