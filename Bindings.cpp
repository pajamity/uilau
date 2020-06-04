/* generated by rust_qt_binding_generator */
#include "Bindings.h"

namespace {
    inline void appDurationMsChanged(App* o)
    {
        Q_EMIT o->durationMsChanged();
    }
    inline void appPositionMsChanged(App* o)
    {
        Q_EMIT o->positionMsChanged();
    }
}
extern "C" {
    App::Private* app_new(App*, void (*)(App*), void (*)(App*));
    void app_free(App::Private*);
    quint64 app_duration_ms_get(const App::Private*);
    quint64 app_position_ms_get(const App::Private*);
    void app_pause(App::Private*);
    void app_play(App::Private*);
    void app_seek_to(App::Private*, quint64);
};

App::App(bool /*owned*/, QObject *parent):
    QObject(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
}

App::App(QObject *parent):
    QObject(parent),
    m_d(app_new(this,
        appDurationMsChanged,
        appPositionMsChanged)),
    m_ownsPrivate(true)
{
}

App::~App() {
    if (m_ownsPrivate) {
        app_free(m_d);
    }
}
quint64 App::durationMs() const
{
    return app_duration_ms_get(m_d);
}
quint64 App::positionMs() const
{
    return app_position_ms_get(m_d);
}
void App::pause()
{
    return app_pause(m_d);
}
void App::play()
{
    return app_play(m_d);
}
void App::seekTo(quint64 to)
{
    return app_seek_to(m_d, to);
}
