/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>

class App;
class Layers;
class TimelineObjects;

class App : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Layers* const m_layers;
    TimelineObjects* const m_objects;
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(quint64 durationMs READ durationMs NOTIFY durationMsChanged FINAL)
    Q_PROPERTY(Layers* layers READ layers NOTIFY layersChanged FINAL)
    Q_PROPERTY(TimelineObjects* objects READ objects NOTIFY objectsChanged FINAL)
    Q_PROPERTY(quint64 positionMs READ positionMs NOTIFY positionMsChanged FINAL)
    explicit App(bool owned, QObject *parent);
public:
    explicit App(QObject *parent = nullptr);
    ~App();
    quint64 durationMs() const;
    const Layers* layers() const;
    Layers* layers();
    const TimelineObjects* objects() const;
    TimelineObjects* objects();
    quint64 positionMs() const;
    Q_INVOKABLE void moveTimelineObject(const QString& obj_name, quint64 dst_layer_id, float dst_time_ms);
    Q_INVOKABLE void pause();
    Q_INVOKABLE void play();
    Q_INVOKABLE void seekTo(quint64 to);
    Q_INVOKABLE void timelineAddFileObject(const QString& file_urls, quint64 dst_layer_id, float dst_time_ms);
    Q_INVOKABLE void timelineApplyObjectFilter(const QString& obj_name, const QString& description);
    Q_INVOKABLE void timelineChangeObjectInpoint(const QString& obj_name, float inpoint_ms);
    Q_INVOKABLE void timelineChangeObjectOutpoint(const QString& obj_name, float outpoint_ms);
    Q_INVOKABLE void timelineConfigureFilter(const QString& obj_name, quint64 dst_layer_id, float dst_time_ms);
    Q_INVOKABLE void timelineConfigureText(const QString& obj_name, quint64 dst_layer_id, float dst_time_ms, const QString& text);
    Q_INVOKABLE void timelineRemoveObject(const QString& obj_name);
Q_SIGNALS:
    void durationMsChanged();
    void layersChanged();
    void objectsChanged();
    void positionMsChanged();
};

class Layers : public QAbstractItemModel
{
    Q_OBJECT
    friend class App;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit Layers(bool owned, QObject *parent);
public:
    explicit Layers(QObject *parent = nullptr);
    ~Layers();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
};

class TimelineObjects : public QAbstractItemModel
{
    Q_OBJECT
    friend class App;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit TimelineObjects(bool owned, QObject *parent);
public:
    explicit TimelineObjects(QObject *parent = nullptr);
    ~TimelineObjects();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE quint64 durationMs(int row) const;
    Q_INVOKABLE QString kind(int row) const;
    Q_INVOKABLE quint64 layerId(int row) const;
    Q_INVOKABLE quint64 maxDurationMs(int row) const;
    Q_INVOKABLE QString name(int row) const;
    Q_INVOKABLE quint64 startMs(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
};
#endif // BINDINGS_H
