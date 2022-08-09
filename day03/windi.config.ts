import { defineConfig } from "windicss/helpers";
import colors from "windicss/colors";

export default defineConfig({
    theme: {
        extend: {
            colors: {
                gray: colors.stone,
            },
        },
    },
});
