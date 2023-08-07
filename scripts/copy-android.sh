for LIBRARY_NAME in bip39 lightning_message
do
    CURR_VERSION=${LIBRARY_NAME}-v`awk '/^version: /{print $2}' packages/${LIBRARY_NAME}/pubspec.yaml`
    cp platform-build/${LIBRARY_NAME}/android.tar.gz packages/flutter_${LIBRARY_NAME}/android/$CURR_VERSION.tar.gz
done