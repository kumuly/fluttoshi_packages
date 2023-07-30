for LIBRARY_NAME in unified_mnemonic lightning_message
do
    CURR_VERSION=${LIBRARY_NAME}-v`awk '/^version: /{print $2}' packages/${LIBRARY_NAME}/pubspec.yaml`
    CAMEL_CASE_LIBRARY_NAME=$(echo "${LIBRARY_NAME}" | awk 'BEGIN{FS=OFS="_"}{for(i=1;i<=NF;i++){sub(".",toupper(substr($i,1,1)),$i)}}1' | sed 's/_//g')
    cp platform-build/${LIBRARY_NAME}/${CAMEL_CASE_LIBRARY_NAME}.xcframework.zip packages/flutter_${LIBRARY_NAME}/ios/Frameworks/$CURR_VERSION.zip
    cp platform-build/${LIBRARY_NAME}/${CAMEL_CASE_LIBRARY_NAME}.xcframework.zip packages/flutter_${LIBRARY_NAME}/macos/Frameworks/$CURR_VERSION.zip
done