/* A plugin written in C: a button in the header that opens a window.
 *
 * It is here to show that the boundary really is C, not Rust wearing a C hat.
 * Nothing in it links Rust, and nothing in it draws: the window is a JSON
 * document handed to the application, which builds it with whatever frontend
 * is running — GTK, the terminal UI or React in a browser.
 *
 * Build it with ../../build-c.sh, then point ic-plugin-check at the result.
 */

#include "ic_plugin.h"

#include <stdio.h>
#include <string.h>

#define PLUGIN_ID "hello-c"
#define VIEW_ID "hello-c"

/* The one table the application gave us, kept for as long as we run. It is
 * the application's own and outlives this plugin, so keeping the pointer is
 * the intended thing to do — copying the struct would not be. */
static const IcHost *host = NULL;

/* What the window shows, and what the button in it changes. */
static unsigned long pressed = 0;

/* Every answer to the application is bytes it reads before calling us again,
 * so one buffer per purpose is enough. */
static char described[2048];
static char replied[256];

static const char *const DOCUMENT_FORMAT =
    "{"
    "\"schema\":1,"
    "\"data\":{\"count\":\"%lu\"},"
    "\"fields\":[],"
    "\"form\":{"
    "\"t\":\"view\",\"surface\":\"dialog\",\"spacing\":12,\"padding\":24,"
    "\"width\":420,\"height\":220,"
    "\"children\":["
    "{\"t\":\"text\",\"id\":\"title\",\"role\":\"title1\","
    "\"text\":{\"tr\":\"helloc.title\",\"en\":\"Hello from C\"}},"
    "{\"t\":\"text\",\"id\":\"said\",\"role\":\"dim\","
    "\"text\":{\"tr\":\"helloc.said\",\"en\":\"This window was described by a plugin "
    "written in C. Nothing here draws anything.\"}},"
    "{\"t\":\"text\",\"id\":\"count\",\"bind\":\"data.count\"},"
    "{\"t\":\"row\",\"spacing\":8,\"children\":["
    "{\"t\":\"button\",\"id\":\"press\","
    "\"title\":{\"tr\":\"helloc.press\",\"en\":\"Press me\"},"
    "\"intent\":{\"do\":\"emit\",\"node\":\"press\"}},"
    "{\"t\":\"button\",\"id\":\"close\","
    "\"title\":{\"tr\":\"helloc.close\",\"en\":\"Close\"},"
    "\"intent\":{\"do\":\"close\"}}"
    "]}"
    "]}"
    "}";

static const char *const ENGLISH =
    "{\"helloc.title\":\"Hello from C\","
    "\"helloc.said\":\"This window was described by a plugin written in C. Nothing here "
    "draws anything.\","
    "\"helloc.press\":\"Press me\","
    "\"helloc.close\":\"Close\","
    "\"helloc.button\":\"C\","
    "\"helloc.tooltip\":\"A window described by a plugin written in C\"}";

static const char *const RUSSIAN =
    "{\"helloc.title\":\"Привет из C\","
    "\"helloc.said\":\"Это окно описал плагин на C. Здесь ничего не рисуется.\","
    "\"helloc.press\":\"Нажми меня\","
    "\"helloc.close\":\"Закрыть\","
    "\"helloc.button\":\"C\","
    "\"helloc.tooltip\":\"Окно, описанное плагином на C\"}";

static const char *const ICON =
    "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 16 16\">"
    "<text x=\"3\" y=\"12\" font-size=\"12\">C</text></svg>";

static IcBytes answer(const char *text) {
    IcBytes out;
    out.data = (const uint8_t *)text;
    out.len = (uint64_t)strlen(text);
    return out;
}

/* The application asks for the window every time it opens one, so the count
 * shown is whatever it is now. */
static IcBytes describe(const uint8_t *ctx, uint64_t ctx_len, void *user_data) {
    (void)ctx;
    (void)ctx_len;
    (void)user_data;
    snprintf(described, sizeof described, DOCUMENT_FORMAT, pressed);
    return answer(described);
}

/* Which node the application says was activated.
 *
 * A real plugin would hand this to a JSON parser; the point here is that
 * answering in JSON needs nothing at all, and reading it is the only part
 * that costs a C plugin anything. */
static int event_is_for(const uint8_t *event, uint64_t len, const char *node) {
    char wanted[64];
    size_t wanted_len;
    size_t at;

    if (event == NULL || len == 0) {
        return 0;
    }
    /* The frontends write this without spaces; a plugin that wants to be sure
     * should parse rather than search. */
    snprintf(wanted, sizeof wanted, "\"node\":\"%s\"", node);
    wanted_len = strlen(wanted);
    if ((uint64_t)wanted_len > len) {
        return 0;
    }
    for (at = 0; at + wanted_len <= (size_t)len; at++) {
        if (memcmp(event + at, wanted, wanted_len) == 0) {
            return 1;
        }
    }
    return 0;
}

static IcBytes on_event(const uint8_t *event, uint64_t event_len, void *user_data) {
    (void)user_data;
    if (!event_is_for(event, event_len, "press")) {
        return answer("{}");
    }
    pressed += 1;
    snprintf(replied, sizeof replied, "{\"set\":{\"data.count\":\"%lu\"}}", pressed);
    return answer(replied);
}

static const IcViewVTable VIEW = {
    (uint32_t)sizeof(IcViewVTable),
    describe,
    on_event,
    NULL,
};

static void header_button_pressed(void *user_data, void *parent_window) {
    (void)user_data;
    (void)parent_window;
    if (host != NULL) {
        host->open_view(VIEW_ID, NULL, 0);
    }
}

static const IcAbout ABOUT = {
    IC_ABOUT_MAGIC,
    (uint32_t)sizeof(IcAbout),
    IC_ABI_VERSION,
    PLUGIN_ID,
    "Hello from C",
    "0.1.0",
    "A header button that opens a window, written in C",
};

const IcAbout *ic_plugin_about(void) { return &ABOUT; }

const char *ic_plugin_name(void) { return ABOUT.name; }

const char *ic_plugin_version(void) { return ABOUT.version; }

int ic_plugin_init(const IcHost *given, const char *kind) {
    /* A window of buttons is a desktop thing; in a terminal there is nothing
     * here worth loading. */
    if (kind != NULL && strcmp(kind, IC_HOST_CONSOLE) == 0) {
        return IC_ERR_INIT_FAILED;
    }
    /* register_locales sits further along the table than anything else this
     * plugin calls, so a host that reaches it reaches all of them. */
    if (given == NULL || given->magic != IC_HOST_MAGIC) {
        return IC_ERR_HOST_UNKNOWN;
    }
    if (!ic_host_usable(given, IC_NEEDS_UP_TO(register_locales))) {
        return IC_ERR_HOST_TOO_OLD;
    }
    host = given;

    host->register_locales("en", (const uint8_t *)ENGLISH, (uint64_t)strlen(ENGLISH));
    host->register_locales("ru", (const uint8_t *)RUSSIAN, (uint64_t)strlen(RUSSIAN));

    if (host->register_view(VIEW_ID, "helloc.title", &VIEW, NULL) != IC_OK) {
        return IC_ERR_INIT_FAILED;
    }
    if (host->add_header_button("hello-c.open", ICON, "helloc.button", "helloc.tooltip",
                                IC_SIDE_RIGHT, 0, header_button_pressed, NULL) != IC_OK) {
        return IC_ERR_INIT_FAILED;
    }
    return IC_OK;
}

void ic_plugin_shutdown(void) { host = NULL; }
