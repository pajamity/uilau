/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>

class App;

class App : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(quint64 durationMs READ durationMs NOTIFY durationMsChanged FINAL)
    Q_PROPERTY(quint64 positionMs READ positionMs NOTIFY positionMsChanged FINAL)
    explicit App(bool owned, QObject *parent);
public:
    explicit App(QObject *parent = nullptr);
    ~App();
    quint64 durationMs() const;
    quint64 positionMs() const;
    Q_INVOKABLE void moveTimelineObject(const QString& object_id, quint64 dst_layer_id, float dst_time_ms) const;
    Q_INVOKABLE void pause();
    Q_INVOKABLE void play();
    Q_INVOKABLE void seekTo(quint64 to);
Q_SIGNALS:
    void durationMsChanged();
    void positionMsChanged();
};
#endif // BINDINGS_H
